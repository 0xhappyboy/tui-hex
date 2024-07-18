use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::StatefulWidget,
};

use crate::HexMatrixState::HexMatrixState;

pub struct HexMatrix {
    label: String,
    style: Style,
}

impl HexMatrix {
    pub fn new(l: String) -> Self {
        Self {
            label: l,
            style: Style::default().bg(Color::Blue),
        }
    }
}

impl StatefulWidget for HexMatrix {
    type State = HexMatrixState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_string(
            area.left(),
            area.top(),
            format!("{}", self.label),
            self.style,
        );
    }
}
