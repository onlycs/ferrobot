use std::io;
use std::{backtrace::Backtrace, panic::Location};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("At {location}: IO Error: {source}")]
    IO {
        #[from]
        source: io::Error,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },

    #[error("Command {command:?} completed unsuccessfully")]
    Command { command: Vec<String> },
}

pub type TaskResult<T = ()> = Result<T, TaskError>;
