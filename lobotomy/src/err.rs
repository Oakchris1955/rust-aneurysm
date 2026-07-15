use std::error::Error;

use aneurysm_lib::interpreter::InterpreterError;

/// Map an error that is thrown by [Interpreter](aneurysm_lib::interpreter::Interpreter)
/// to a [Box]ed dynamic [Error]
pub(crate) fn map_interpreter_err(err: InterpreterError) -> Box<dyn Error> {
    match err {
        InterpreterError::IOError(io_err) => Box::new(io_err),
        _ => unreachable!(),
    }
}
