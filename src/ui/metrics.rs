use crate::state::State;
use ratatui::{
    prelude::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Padding, Paragraph},
    Frame,
};
use std::sync::Arc;

pub fn render(frame: &mut Frame, state: &Arc<State>, area: Rect) {
    let request_total = state.request_total.load().to_string();
    let request_second = state.request_second.load().to_string();
    let transfer_total = format!("{} MB", state.transfer_total.load() / 1024 / 1024);

    let areas = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ],
    )
    .split(area);

    let request_total_widget = metric_widget("Request Total", &request_total);

    frame.render_widget(request_total_widget, areas[0]);

    let request_second_widget = metric_widget("Request/Second", &request_second);

    frame.render_widget(request_second_widget, areas[1]);

    let transfer_total_widget = metric_widget("Transfer Total", &transfer_total);

    frame.render_widget(transfer_total_widget, areas[2]);
}

fn metric_widget<'a>(title: &'a str, text: &'a str) -> Paragraph<'a> {
    let block = Block::bordered()
        .title(title)
        .padding(Padding::horizontal(1));
    Paragraph::new(text).block(block)
}
