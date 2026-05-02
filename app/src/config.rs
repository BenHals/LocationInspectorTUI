use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct Config {
    pub ui: UiConfig,
    pub data: DataConfig,

    #[serde(default)]
    pub layers: Vec<LayerConfig>,
}

#[derive(serde::Deserialize)]
pub struct UiConfig {
    pub region_label: String,
}

#[derive(serde::Deserialize)]
pub struct DataConfig {
    pub root_dir: PathBuf,
}

#[derive(serde::Deserialize, Clone)]
pub struct LayerConfig {
    pub id: String,
    pub name: String,
    pub command: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ui: UiConfig {
                region_label: "Region".to_string(),
            },
            data: DataConfig {
                root_dir: "app/data/example_data".into(),
            },
            layers: vec![],
        }
    }
}
