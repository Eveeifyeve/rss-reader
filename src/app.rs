use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Viewing,
    Adding,
    Editing,
    Searching,
    Exiting,
}

#[derive(PartialEq)]
pub enum CurrentlyEditing {
    FeedUrl,
    Piority,
}

pub struct App {
    pub pairs: HashMap<String, u8>,
    pub feed_url: String,
    pub piority: u8,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> App {
        App {
            pairs: HashMap::new(),
            piority: 0,
            feed_url: String::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn toggle_editing(&mut self) {
        if self.currently_editing == Some(CurrentlyEditing::FeedUrl) {
            self.currently_editing = None;
        } else {
            self.currently_editing = Some(CurrentlyEditing::FeedUrl);
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.feed_url.clone(), self.piority.clone());

        self.feed_url = String::new();
        self.piority = 0;
        self.currently_editing = None;
    }
}
