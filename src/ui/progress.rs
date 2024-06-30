use crate::state::State;
use ratatui::{
    prelude::{Alignment, Rect},
    widgets::{Block, Gauge},
    Frame,
};
use std::sync::Arc;

pub fn render(frame: &mut Frame, state: &Arc<State>, area: Rect) {
    let elapsed = state.start.elapsed().as_millis();
    let duration = state.duration.as_millis();
    let percent = (elapsed * 100 / duration).min(100);

    let block = Block::bordered()
        .title("Progress")
        .title_alignment(Alignment::Center);
    let progress = Gauge::default().block(block).percent(percent as u16);

    frame.render_widget(progress, area);
}
