use git2;
use reqwest;
use serde_json;
use std;
use std::error;
use std::fmt;
use toml;

#[derive(Debug)]
pub struct IMErr {
    msg: String,
}

impl fmt::Display for IMErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for IMErr {
    fn description(&self) -> &str {
        &self.msg
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(self)
    }
}

#[derive(Debug)]
pub enum IMError {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    Git(git2::Error),
    Var(std::env::VarError),
    De(toml::de::Error),
    SerdeJson(serde_json::Error),
    IM(IMErr),
}

pub type IMResult<T> = Result<T, IMError>;

impl IMError {
    pub fn new(msg: &str) -> IMError {
        IMError::IM(IMErr {
            msg: String::from(msg),
        })
    }
}

impl fmt::Display for IMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IMError::Reqwest(ref err) => write!(f, "Error in request: {}", err),
            IMError::Io(ref err) => write!(f, "IO error: {}", err),
            IMError::Git(ref err) => write!(f, "git error: {}", err),
            IMError::Var(ref err) => write!(f, "Environment error: {}", err),
            IMError::De(ref err) => write!(f, "Deserialization error: {}", err),
            IMError::SerdeJson(ref err) => write!(f, "JSON error: {}", err),
            IMError::IM(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for IMError {
    fn description(&self) -> &str {
        match *self {
            IMError::Reqwest(ref err) => err.description(),
            IMError::Io(ref err) => err.description(),
            IMError::Git(ref err) => err.description(),
            IMError::Var(ref err) => err.description(),
            IMError::De(ref err) => err.description(),
            IMError::SerdeJson(ref err) => err.description(),
            IMError::IM(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            IMError::Reqwest(ref err) => Some(err),
            IMError::Io(ref err) => Some(err),
            IMError::Git(ref err) => Some(err),
            IMError::Var(ref err) => Some(err),
            IMError::De(ref err) => Some(err),
            IMError::SerdeJson(ref err) => Some(err),
            IMError::IM(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for IMError {
    fn from(err: std::io::Error) -> IMError {
        IMError::Io(err)
    }
}

impl From<reqwest::Error> for IMError {
    fn from(err: reqwest::Error) -> IMError {
        IMError::Reqwest(err)
    }
}

impl From<git2::Error> for IMError {
    fn from(err: git2::Error) -> IMError {
        IMError::Git(err)
    }
}

impl From<std::env::VarError> for IMError {
    fn from(err: std::env::VarError) -> IMError {
        IMError::Var(err)
    }
}

impl From<toml::de::Error> for IMError {
    fn from(err: toml::de::Error) -> IMError {
        IMError::De(err)
    }
}

impl From<serde_json::Error> for IMError {
    fn from(err: serde_json::Error) -> IMError {
        IMError::SerdeJson(err)
    }
}

impl From<IMErr> for IMError {
    fn from(err: IMErr) -> IMError {
        IMError::IM(err)
    }
}
