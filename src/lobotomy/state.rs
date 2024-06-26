use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use aneurysm::interpreter::*;

pub struct State<'a, 'b> {
    pub interpreter: Interpreter<'a, 'b>,
    pub breakpoints: Vec<usize>,
    pub filepath: PathBuf,
}

impl<'a, 'b> State<'a, 'b> {
    pub fn new(interpreter: Interpreter<'a, 'b>, filepath: PathBuf) -> Self {
        State {
            interpreter,
            breakpoints: Vec::new(),
            filepath,
        }
    }

    pub fn filename(&self) -> String {
        self.filepath
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
    }
}

pub struct Prompt<'a, 'b> {
    state: Rc<RefCell<State<'a, 'b>>>,
}

impl<'a, 'b> Prompt<'a, 'b> {
    pub fn new(state: Rc<RefCell<State<'a, 'b>>>) -> Self {
        Prompt { state }
    }
}

impl<'a, 'b> std::fmt::Display for Prompt<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "&\x1B[1m{}\x1B[0m> ", self.state.borrow().filename())
    }
}
