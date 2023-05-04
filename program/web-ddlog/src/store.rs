use yewdux::store::Store;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct State {
    count: i32,
    program: String
}


impl State {
    pub fn inc(&mut self) {
        self.count += 1
    }

    pub fn dec(&mut self) {
        self.count -= 1
    }
    pub fn get_count(&self) -> i32 {
        self.count
    }

    pub fn get_program(&self) -> String {
        self.program.clone()
    }

    pub fn set_program(&mut self, program: String) {
        self.program = program
    }
}