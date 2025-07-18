use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize, palette::tailwind},
    text::Line,
    widgets::{Block, Borders, Gauge, Padding, Paragraph, Widget},
};

const BATGAUGE_COLOR: Color = tailwind::ORANGE.c800;
const LABEL_COLOR: Color = tailwind::SLATE.c200;

#[derive(Default)]
pub struct Battery {}

impl Battery {
    pub fn render(area: Rect, buf: &mut Buffer) {
        let status_title = title_block("Battery Status");
        Paragraph::new(create_status(area))
            .block(status_title)
            .render(area, buf);

        let gauge_area = Rect::new(area.x, area.y + 8, area.width / 2, 4);
        let title = title_block("Battery Percentage:");
        Gauge::default()
            .block(title)
            .gauge_style(BATGAUGE_COLOR)
            .percent(76)
            .render(gauge_area, buf);
    }
}

fn title_block(title: &str) -> Block {
    let title = Line::from(title);
    Block::new()
        .borders(Borders::NONE)
        .padding(Padding::vertical(1))
        .title(title)
        .fg(LABEL_COLOR)
}

fn create_status(_area: Rect) -> Vec<Line<'static>> {
    vec![
        Line::raw("State:               Charging"),
        Line::raw("Present Rate:        100mAh"),
        Line::raw("Remaining Capacity:  2000mAh"),
        Line::raw("Present Voltage:     7543mV"),
    ]
}
