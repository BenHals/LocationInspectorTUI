use std::{collections::HashSet, sync::mpsc};

use ratatui::Frame;

use crate::{
    component::Component,
    config::Config,
    db::file_db::FileDB,
    layers::spawn_layer_load,
    message::Message,
    model::{InteractionMode, LayerState, Model},
    update::Update,
    view::View,
};

pub struct App {
    pub model: Model,
    pub view: View,
    pub db: FileDB,
    pub async_tx: mpsc::Sender<Update>,
}

impl App {
    pub fn new(db: FileDB, config: Config, async_tx: mpsc::Sender<Update>) -> Self {
        Self {
            model: Model::new(config),
            view: View::new(&db),
            db,
            async_tx,
        }
    }

    pub fn handle(&mut self, msg: Message) {
        self.model.err = None;
        let mut queue: Vec<Message> = vec![msg];
        while let Some(m) = queue.pop() {
            if let Message::LoadLayers = m {
                self.spawn_all_layer_loads();
                continue;
            }
            let (updates, follow_ups) = self.view.update(&m, &self.model, &self.db);
            for u in updates {
                self.model.apply(u);
            }
            queue.extend(follow_ups);
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        self.view.render(frame, area, &self.model);
    }

    fn spawn_all_layer_loads(&mut self) {
        let InteractionMode::InspectingLocation { state } = &self.model.interaction_mode else {
            return;
        };
        let location_id = state.location.tag.id.clone();
        let region_ids: Vec<String> = state
            .location
            .polygons
            .iter()
            .map(|p| p.metadata.id.clone())
            .collect();
        let layers = self.model.config.layers.clone();
        let data_root = self.model.config.data.root_dir.clone();
        let already_triggered: HashSet<String> = state
            .layers
            .iter()
            .filter(|(_, s)| {
                matches!(
                    s,
                    LayerState::Loading | LayerState::Loaded(_) | LayerState::Failed(_)
                )
            })
            .map(|(id, _)| id.clone())
            .collect();
        for layer_config in layers {
            if already_triggered.contains(&layer_config.id) {
                continue;
            }
            self.model.apply(Update::TriggerLayerLoad {
                location_id: location_id.clone(),
                layer_id: layer_config.id.clone(),
            });
            spawn_layer_load(
                layer_config,
                location_id.clone(),
                region_ids.clone(),
                data_root.clone(),
                self.async_tx.clone(),
            );
        }
    }
}
