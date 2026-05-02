use std::{
    collections::HashMap,
    error::Error,
    io::Write,
    process::{Command, Stdio},
    sync::mpsc,
};

use crate::{config::LayerConfig, update::Update};

pub fn spawn_layer_load(
    config: LayerConfig,
    location_id: String,
    region_ids: Vec<String>,
    tx: mpsc::Sender<Update>,
) {
    std::thread::spawn(move || {
        let update = match run_layer_command(&config, &location_id, &region_ids) {
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
) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let mut child = Command::new(&config.command)
        .env("LOCTUI_LOCATION_ID", location_id)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().ok_or("Could not open stdin")?;
        let payload = serde_json::to_string(region_ids)?;
        stdin.write_all(payload.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Layer command failed {}", stderr.trim()).into());
    }

    let layer_data: HashMap<String, f64> = serde_json::from_slice(&output.stdout)?;
    Ok(layer_data)
}
