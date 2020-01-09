use crate::intcode::{
    error::IntcodeErr,
    interpreter::{Int, Intcode, State},
};
use std::iter;

// macro_rules! intcode {
//     ($ic:ident {$code:block}) => ({
//         '_ic: loop {
//             $code
//         }
//     })
// }

/// Consume the intcode
#[macro_export]
macro_rules! inp {
    ($ic:ident, $v:expr) => ({
        loop {
            match $ic.step() {
                crate::intcode::interpreter::State::Running => continue,
                crate::intcode::interpreter::State::Output(_) => continue,
                crate::intcode::interpreter::State::Waiting => {
                    $ic.input($v.into());
                    break true
                }
                crate::intcode::interpreter::State::Halted => break false,
                crate::intcode::interpreter::State::Error(e) => Err(e)?,
            }
        }
    })
}

#[macro_export]
macro_rules! out {
    ($ic:ident) => ({
        loop {
            match $ic.step() {
                crate::intcode::interpreter::State::Running => continue,
                crate::intcode::interpreter::State::Output(o) => break Some(o.into()),
                crate::intcode::interpreter::State::Waiting => Err(crate::intcode::error::IntcodeErr::EvalNoArgs)?,
                crate::intcode::interpreter::State::Halted => break None,
                crate::intcode::interpreter::State::Error(e) => Err(e)?,
            }
        }
    })
}

#[macro_export]
macro_rules! out_or {
    ($ic:ident) => ({
        let res = loop {
            match $ic.step() {
                crate::intcode::interpreter::State::Running => continue,
                crate::intcode::interpreter::State::Output(o) => break Some(o.into()),
                crate::intcode::interpreter::State::Waiting => Err(crate::intcode::error::IntcodeErr::EvalNoArgs)?,
                crate::intcode::interpreter::State::Halted => break None,
                crate::intcode::interpreter::State::Error(e) => Err(e)?,
            };
        };
        match res {
            Some(e) => e,
            None => break,
        }
    })
}

/// Fully evaluate the intcode instance `ic` to completion
/// Stops with an `Err(IntcodeErr::EvalNoArgs)` when it can't provide an argument
/// Stops with an `Err(E)` when the interpreter errors with code E
/// Otherwise returns a Vec<Int> of all the outputs before halting
pub fn eval<I>(ic: &mut Intcode, mut inp: I) -> Result<Vec<Int>, IntcodeErr>
where
    I: Iterator<Item = Int>,
{
    let mut out = Vec::new();
    loop {
        match ic.step() {
            State::Running => continue,
            State::Output(o) => out.push(o),
            State::Waiting => ic.input(inp.next().ok_or(IntcodeErr::EvalNoArgs)?),
            State::Halted => break,
            State::Error(e) => return Err(e),
        }
    }
    Ok(out)
}

/// Evaluate an intcode instance with all inputs as `inp`
/// Returns `Some(out)` on the first output
/// Will return `None` if it stops running before an output is produced
pub fn eval_once(ic: &mut Intcode, inp: Int) -> Result<Int, IntcodeErr> {
    match eval(ic, iter::once(inp)).map(|v| v.get(0).copied()) {
        Ok(None) => Err(IntcodeErr::EvalNoArgs),
        Ok(Some(i)) => Ok(i),
        Err(e) => Err(e),
    }
}

/// Like eval_once but uses an argument vector instead of a single arg
/// Will return `None` if the interpreter asks for too many inputs
pub fn eval_args(ic: &mut Intcode, argv: impl Iterator<Item = Int>) -> Result<Int, IntcodeErr> {
    match eval(ic, argv).map(|v| v.get(0).copied()) {
        Ok(None) => Err(IntcodeErr::EvalNoArgs),
        Ok(Some(i)) => Ok(i),
        Err(e) => Err(e),
    }
}

/// Like eval_once but returns
pub fn eval_all(ic: &mut Intcode, arg: Int) -> Result<Vec<Int>, IntcodeErr> {
    eval(ic, iter::once(arg))
}
