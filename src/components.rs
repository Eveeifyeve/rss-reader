use ratatui::{prelude::*, text::*, widgets::*};

pub fn block(title: &str, style: Option<Style>, borders: Option<Borders>) -> Block {
    Block::default()
        .title(title)
        .borders(borders.unwrap_or_else(|| Borders::NONE))
        .style(style.unwrap_or_else(|| Style::default()))
}

pub fn style(bg: Option<Color>, fg: Option<Color>) -> Style {
    Style::default()
        .bg(bg.unwrap_or_else(|| Color::Black))
        .fg(fg.unwrap_or_else(|| Color::Black))
}

pub fn text(text: &str, style: Option<Style>) -> Text {
    Text::styled(text, style.unwrap_or_else(|| Style::default()))
}

pub fn span(text: &str, style: Option<Style>) -> Span {
    Span::styled(text, style.unwrap_or_else(|| Style::default()))
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
