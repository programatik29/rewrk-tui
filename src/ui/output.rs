use crate::state::State;
use ratatui::{
    prelude::Rect,
    widgets::{Block, Padding, Paragraph},
    Frame,
};
use std::{fmt::Write, sync::Arc};

pub fn render(frame: &mut Frame, state: &Arc<State>, area: Rect) {
    let text = state
        .errors
        .lock()
        .map
        .iter()
        .fold(String::new(), |mut s, (k, v)| {
            writeln!(&mut s, "> Errors({v}): {k}").unwrap();
            s
        });

    let block = Block::bordered()
        .title("Output")
        .padding(Padding::horizontal(1));

    let output = Paragraph::new(text).block(block);

    frame.render_widget(output, area)
}
