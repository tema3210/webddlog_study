use std::{collections::{HashMap, hash_map::Entry}};

use yewdux::store::Store;

#[derive(Default, Clone, Debug)]
pub struct Program {
    text: String
}

impl Program {
    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn from_string(s: &str) -> Self {
        Self { text: s.into() }
    }
}

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

#[derive(Default, Clone, PartialEq, Store, Debug)]
pub struct State {
    current_module: Option<String>,
    modules: HashMap<String,Program>
}


impl State {
    pub fn list_modules(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }

    pub fn get_program(&self, key: &str) -> Option<Program> {
        self.modules.get(key).cloned()
    }

    pub fn update_program<F: FnOnce(&Program) -> Program>(&mut self, key: &str, f: F) {
        match self.modules.entry(key.into()) {
            Entry::Occupied(mut e) => {
                e.insert(f(e.get()));
            },
            _ => {}
        }
    }

    pub fn set_current_module(&mut self, tab: &str) {
        self.current_module = Some(tab.into());
    }

    pub fn get_current_module(&self) -> Option<String> {
        self.current_module.clone()
    }

    pub fn add_empty_module(&mut self, name: &str) {
        self.modules.insert(name.into(), Program { text: String::new() });
        self.current_module = Some(name.into());
    }
}