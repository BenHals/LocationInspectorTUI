use std::{
    collections::HashMap,
    error::Error,
    io::{Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    sync::mpsc,
    time::{Duration, Instant},
};

use crate::{config::LayerConfig, update::Update};

const POLL_INTERVAL: Duration = Duration::from_millis(50);

pub fn spawn_layer_load(
    config: LayerConfig,
    location_id: String,
    region_ids: Vec<String>,
    data_root: PathBuf,
    tx: mpsc::Sender<Update>,
) {
    std::thread::spawn(move || {
        let update = match run_layer_command(&config, &location_id, &region_ids, &data_root) {
            Ok(layer_data) => Update::SetLayerData {
                location_id,
                layer_id: config.id,
                layer_data,
            },
            Err(e) => Update::SetLayerFailed {
                location_id,
                layer_id: config.id,
                err_msg: e.to_string(),
            },
        };
        let _ = tx.send(update);
    });
}

pub fn run_layer_command(
    config: &LayerConfig,
    location_id: &str,
    region_ids: &[String],
    data_root: &PathBuf,
) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let mut child = Command::new(&config.command)
        .env("LOCTUI_LOCATION_ID", location_id)
        .env("LOCTUI_DATA_ROOT", data_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().ok_or("Could not open stdin")?;
        let payload = serde_json::to_string(region_ids)?;
        stdin.write_all(payload.as_bytes())?;
    }
    // Close stdin so the child sees EOF and can complete.
    drop(child.stdin.take());

    // Poll for completion, killing the child if the timeout is exceeded.
    // Note: this assumes layer scripts produce small output (well under the
    // OS pipe buffer of ~64KB). Larger outputs would block the child on
    // stdout writes since we don't drain until exit.
    let timeout = Duration::from_secs(config.timeout_secs);
    let start = Instant::now();
    let status = loop {
        match child.try_wait()? {
            Some(s) => break s,
            None => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(format!(
                        "Layer command timed out after {}s",
                        timeout.as_secs()
                    )
                    .into());
                }
                std::thread::sleep(POLL_INTERVAL);
            }
        }
    };

    let mut stdout = Vec::new();
    if let Some(mut s) = child.stdout.take() {
        s.read_to_end(&mut stdout)?;
    }
    let mut stderr = Vec::new();
    if let Some(mut s) = child.stderr.take() {
        s.read_to_end(&mut stderr)?;
    }

    if !status.success() {
        let stderr = String::from_utf8_lossy(&stderr);
        return Err(format!("Layer command failed {}", stderr.trim()).into());
    }

    let layer_data: HashMap<String, f64> = serde_json::from_slice(&stdout)?;
    Ok(layer_data)
}
