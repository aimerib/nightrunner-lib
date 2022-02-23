/// Module for the action parser. [Actions](Actions)
/// are structs containing the important information
/// needed to parse the user input.
pub mod action;
pub mod errors;
/// Module with the various functions used to parse
/// the user input.
pub mod interpreter;

use std::cell::RefCell;
use std::rc::Rc;

use self::action::Action;
use self::errors::{EmptyInput, InvalidAction};
use self::interpreter::process_action;
use crate::config::State;
use crate::NRResult;
use crate::ParsingResult;

/// This is the main function that executes the game.
/// The `NightRunner` struct is the main entry point
/// for the library, and calls this function along with
/// the `State` struct. The return for this function is
/// a `ParsingResult` which is contains the output of
/// the game. The `ParsingResult` returned by this
/// function that is meant to be consumed by the frontend.
pub fn parse(state: Rc<RefCell<State>>, input: &str) -> NRResult<ParsingResult> {
    if !input.is_empty() {
        let action = Action::parse(&*state.borrow(), input);
        match action.is_valid() {
            true => process_action(&state, action),
            false => Err(InvalidAction.into()),
        }
    } else {
        Err(EmptyInput.into())
    }
}

#[cfg(test)]
mod tests;
