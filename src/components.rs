use ratatui::{prelude::*, text::*, widgets::*};

pub fn popup_block(title: &str, style: Style, borders: Borders) -> Block {
    Block::default().title(title).borders(borders).style(style)
}

pub fn text(text: &str, style: Style) -> Text {
    Text::styled(text, style)
}

pub fn style(bg: Color, fg: Color) -> Style {
    Style::default().bg(bg).fg(fg)
}
