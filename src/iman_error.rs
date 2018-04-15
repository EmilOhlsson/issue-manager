use std::error;
use std::fmt;
use reqwest;
use git2;
use std;

#[derive(Debug)]
pub struct IMError {
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
    IM(IMErr),
}

pub type IMResult<T> = Result<T, IMError>;

impl IMError {
    pub fn new(msg: String) -> IMError {
        IMError::IMError(IMError { msg: msg })
    }
}

impl fmt::Display for IMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IMError::Reqwest(ref err) => write!(f, "Error in request: {}", err),
            IMError::Io(ref err) => write!(f, "IO error: {}", err),
            IMError::Git(ref err) => write!(f, "git error: {}", err),
            IMError::IMError(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for IMError {
    fn description(&self) -> &str {
        match *self {
            IMError::Reqwest(ref err) => err.description(),
            IMError::Io(ref err) => err.description(),
            IMError::Git(ref err) => err.description(),
            IMError::IMError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            IMError::Reqwest(ref err) => Some(err),
            IMError::Io(ref err) => Some(err),
            IMError::Git(ref err) => Some(err),
            IMError::IMError(ref err) => Some(err),
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

impl From<IMError> for IMError {
    fn from(err: IMError) -> IMError {
        IMError::IMError(err)
    }
}
