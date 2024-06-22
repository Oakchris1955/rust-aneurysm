use std::cell::RefCell;
use std::rc::Rc;

use aneurysm::interpreter::*;

const DEFAULT_PROMPT: &str = "&> ";

pub struct State<'a, 'b> {
    pub interpreter: Interpreter<'a, 'b>,
    pub breakpoints: Vec<usize>,
    pub prompt: String,
}

impl<'a, 'b> State<'a, 'b> {
    pub fn new(interpreter: Interpreter<'a, 'b>) -> Self {
        State {
            interpreter,
            breakpoints: Vec::new(),
            prompt: DEFAULT_PROMPT.to_string(),
        }
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
        write!(f, "{}", self.state.borrow().prompt)
    }
}
