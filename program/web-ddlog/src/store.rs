use std::{collections::HashMap, rc::Rc};

use yewdux::store::Store;

#[derive(Default, Clone)]
struct Program {
    text: String
}

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

#[derive(Default, Clone, PartialEq, Store)]
pub struct State {
    current_module: String,
    modules: HashMap<String,Program>
}


impl State {
    pub fn list_tabs(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }
    pub fn set_current_tab(&mut self, tab: String) {
        self.current_module = tab
    }

    pub fn get_current_tab(&self) -> String {
        self.current_module.clone()
    }
}