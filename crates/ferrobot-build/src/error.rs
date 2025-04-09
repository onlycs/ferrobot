use std::{backtrace::Backtrace, io, panic::Location};

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

    #[error("At {location}: CBindgen Error: {source}")]
    CBindgen {
        #[from]
        source: cbindgen::Error,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },
}

pub type TaskResult<T = ()> = Result<T, TaskError>;
