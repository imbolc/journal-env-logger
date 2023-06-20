//! # journal-env-logger
//!
//! Logging into system journal based on `RUST_LOG` environment variable
//!
//! ## Usage
//! ```rust,no_run
//! // Initialize logging into journal
//! journal_env_logger::init_journal().unwrap();
//!
//! // Initialize logging into stdout (e.g. for dev purposes)
//! journal_env_logger::init_stdout().unwrap();
//!
//! // A helper to initialize stdout on dev and journal on prod
//! let is_prod = true;
//! journal_env_logger::init(is_prod).unwrap();
//! ```

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

use std::{error, fmt, io};
use tracing_subscriber::{filter::FromEnvError, prelude::*, registry, EnvFilter};

/// Logging initialization error
#[derive(Debug)]
pub enum Error {
    /// can't parse `RUST_LOG` env var
    Env(FromEnvError),
    /// can't connect to Journald
    Journal(io::Error),
}

/// Initialises logging with formatting based on `RUST_LOG` environment variable
/// Writes logs to journal with `to_journal` is true or to `stdout` otherwise
pub fn init(to_journal: bool) -> Result<(), Error> {
    if to_journal {
        init_journal()
    } else {
        init_stdout()
    }
}

/// Initialises logging into journal with formatting based on `RUST_LOG` environment variable
pub fn init_journal() -> Result<(), Error> {
    let filter = env_filter()?;
    let journal_layer = tracing_journald::layer().map_err(Error::Journal)?;
    registry().with(filter).with(journal_layer).init();
    Ok(())
}

/// Initialises logging to stdout (e.g. for dev purposes) with formatting based on `RUST_LOG`
/// environment variable
pub fn init_stdout() -> Result<(), Error> {
    let filter = env_filter()?;
    tracing_subscriber::fmt().with_env_filter(filter).init();
    Ok(())
}

fn env_filter() -> Result<EnvFilter, Error> {
    EnvFilter::try_from_default_env().map_err(Error::Env)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Env(_) => f.write_str("can't parse `RUST_LOG` environment variable"),
            Self::Journal(_) => f.write_str("cat't connect to journald"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Env(ref e) => Some(e),
            Self::Journal(ref e) => Some(e),
        }
    }
}
