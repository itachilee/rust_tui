use std::collections::HashMap;
use std::io::{Result, Write};

use std::fs::{File, OpenOptions};
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,

    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
    // --snip--

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
        self.save_file().expect("msg");
    }
    // --snip--

    pub fn save_file(&self) -> Result<bool> {
        let mut file = OpenOptions::new()
            // .append(true)
            .open("./db.json")
            .expect("failed to open");
        let json_string = serde_json::to_string(&self.pairs)?;
        file.write_all(json_string.as_bytes())?;
        Ok(true)
    }

    // --snip--

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => {
                    self.currently_editing = Some(CurrentlyEditing::Key);
                }
                CurrentlyEditing::Value => {
                    self.currently_editing = Some(CurrentlyEditing::Value);
                }
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    pub fn print_json(&self) -> Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}
