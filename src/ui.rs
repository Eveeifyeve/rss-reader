use crate::components::*;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Color, Style}, // Keep the original Color from prelude
    text::*,
    widgets::*,
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "RSS Reader Feeds",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();

    for key in app.pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let list = List::new(list_items);

    frame.render_widget(list, chunks[1]);

    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => {
                span("Editing Mode", Some(Style::default().fg(Color::Yellow)))
            }
            CurrentScreen::Adding => {
                span("Adding Mode", Some(Style::default().fg(Color::LightRed)))
            }
            CurrentScreen::Searching => span(
                "Searching Mode",
                Some(Style::default().fg(Color::LightGreen)),
            ),
            CurrentScreen::Viewing => {
                span("Viewing Mode", Some(Style::default().fg(Color::LightBlue)))
            }
            CurrentScreen::Exiting => span("Exiting", Some(Style::default().fg(Color::LightRed))),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::FeedUrl => {
                        Span::styled("Editing Feed Url", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Piority => {
                        Span::styled("Editing Piority", Style::default().fg(Color::LightGreen))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            _ => Span::styled("Default Case", Style::default().fg(Color::Gray)),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);
    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    // Editing Mode
    if let Some(editing) = &app.currently_editing {
        let popup_block = block(
            "Enter a new key-value pair",
            Some(Style::default().bg(Color::DarkGray)),
            None,
        );

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut key_block = block("Key", Some(Style::default().fg(Color::Yellow)), None);
        let mut value_block = block("Value", Some(Style::default().fg(Color::Yellow)), None);

        let active_style = style(Some(Color::Yellow), Some(Color::Black));

        match editing {
            CurrentlyEditing::FeedUrl => key_block = key_block.style(active_style),
            CurrentlyEditing::Piority => value_block = value_block.style(active_style),
        };

        let key_text = Paragraph::new(" ")
            .block(key_block)
            .wrap(Wrap { trim: false });
        frame.render_widget(key_text, popup_chunks[0]);

        let value_text = Paragraph::new(app.piority.to_text())
            .block(value_block)
            .wrap(Wrap { trim: false });
        frame.render_widget(value_text, popup_chunks[1]);
    }

    // Exiting Mode
    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = block("Y/N", Some(Style::default().bg(Color::DarkGray)), None);

        let exit_text = text(
            "Would you like to output the buffer as json? (y/n)",
            Some(Style::default().fg(Color::Red)),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}
