use std::time::{Duration, Instant};

use arboard::Clipboard;
use ratatui::{
    Frame, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, Paragraph}
};

use crate::{
    coastlines::coastlines,
    component::Component,
    components::map_view::{MapView, MapViewCtx},
    db::file_db::FileDB,
    domain::{geometry::WGS84, location::Location},
    message::Message,
    model::InspectingLocationView,
    update::Update,
};

pub struct SummaryScreenCtx<'a> {
    pub location: &'a Location,
    pub err: &'a Option<String>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum CopiedField {
    Id,
    Name,
    Country,
    Coord,
}

const FLASH_DURATION: Duration = Duration::from_millis(300);

pub struct SummaryScreen {
    pub map: MapView<WGS84>,
    flash: Option<(CopiedField, Instant)>,
}

impl SummaryScreen {
    pub fn new() -> Self {
        Self {
            map: MapView::new(coastlines(), Some(0.1), true, true),
            flash: None,
        }
    }

    fn copy(&mut self, field: CopiedField, value: &str) {
        if let Ok(mut cb) = Clipboard::new() {
            if cb.set_text(value).is_ok() {
                self.flash = Some((field, Instant::now() + FLASH_DURATION));
            }
        }
    }

    fn flash_style(&self, field: CopiedField) -> Style {
        match self.flash {
            Some((f, until)) if f == field && Instant::now() < until => {
                Style::new().bg(Color::Green).fg(Color::Black)
            }
            _ => Style::new(),
        }
    }
}

impl Component for SummaryScreen {
    type Ctx<'a> = SummaryScreenCtx<'a>;
    fn update(
        &mut self,
        msg: &Message,
        ctx: SummaryScreenCtx,
        db: &FileDB,
    ) -> (Vec<Update>, Vec<Message>) {
        match msg {
            Message::Esc => return (vec![Update::ClearLocation], vec![]),
            Message::Tab => {
                return (
                    vec![Update::SetInspectingLocationView(
                        InspectingLocationView::InspectScreen,
                    )],
                    vec![Message::Activated, Message::LoadLayers],
                )
            }
            Message::Char('i') => {
                self.copy(CopiedField::Id, &ctx.location.tag.id);
                return (vec![], vec![]);
            }
            Message::Char('n') => {
                self.copy(CopiedField::Name, &ctx.location.tag.name);
                return (vec![], vec![]);
            }
            Message::Char('c') => {
                let value = format!(
                    "{}/{}",
                    ctx.location.tag.country_code, ctx.location.tag.country_subdivision
                );
                self.copy(CopiedField::Country, &value);
                return (vec![], vec![]);
            }
            Message::Char('l') => {
                let coord = &ctx.location.tag.coord;
                let value = format!("{:.4}, {:.4}", coord.x, coord.y);
                self.copy(CopiedField::Coord, &value);
                return (vec![], vec![]);
            }
            _ => (),
        }
        let map_ctx = MapViewCtx {
            center: &ctx.location.latlng,
            boundaries: &[],
            regions: &[],
            polylines: &[],
            points: &[],
            title: &ctx.location.tag.name,
            selected_region: &None,
            fill_info: None,
        };
        self.map.update(msg, map_ctx, db)
    }

    fn render<'a>(&self, frame: &mut Frame, area: Rect, ctx: SummaryScreenCtx<'a>) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Min(10)])
            .split(area);

        let map_ctx = MapViewCtx {
            center: &ctx.location.latlng,
            boundaries: &[],
            regions: &[],
            polylines: &[],
            points: &[],
            title: "None",
            selected_region: &None,
            fill_info: None,
        };
        self.map.render(frame, layout[1], map_ctx);

        let summary_block = Block::bordered().title("Summary");
        frame.render_widget(&summary_block, layout[0]);
        let err_str = match &ctx.err {
            Some(err) => format!(" - {}", err),
            _ => String::new(),
        };

        let tag = &ctx.location.tag;
          let dim = Style::new().add_modifier(Modifier::DIM);
          let key = Style::new().yellow();

          let lines = vec![
              Line::from(vec![
                  Span::styled("[i]", key), Span::raw(" "),
                  Span::styled("ID:       ", dim),
                  Span::styled(&tag.id, self.flash_style(CopiedField::Id)),
              ]),
              Line::from(vec![
                  Span::styled("[n]", key), Span::raw(" "),
                  Span::styled("Name:     ", dim),
                  Span::styled(&tag.name, self.flash_style(CopiedField::Name)),
              ]),
              Line::from(vec![
                  Span::styled("[c]", key), Span::raw(" "),
                  Span::styled("Country:  ", dim),
                  Span::styled(
                      format!("{}/{}", tag.country_code, tag.country_subdivision),
                      self.flash_style(CopiedField::Country),
                  ),
              ]),
              Line::from(vec![
                  Span::styled("   ", dim),
                  Span::styled("Type:     ", dim), Span::raw(&tag.kind),
              ]),
              Line::from(vec![
                  Span::styled("   ", dim),
                  Span::styled("Status:   ", dim), Span::raw(&tag.status),
              ]),
              Line::from(vec![
                  Span::styled("   ", dim),
                  Span::styled("Created:  ", dim),
                  Span::raw(tag.created_date.get(..10).unwrap_or(&tag.created_date)),
              ]),
              Line::from(vec![
                  Span::styled("[l]", key), Span::raw(" "),
                  Span::styled("Coord:    ", dim),
                  Span::styled(
                      format!("{:.4}, {:.4}", tag.coord.x, tag.coord.y),
                      self.flash_style(CopiedField::Coord),
                  ),
              ]),
          ];

        let p = Paragraph::new(lines);
        frame.render_widget(p, summary_block.inner(layout[0]));
    }
}
