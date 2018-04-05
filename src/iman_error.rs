use std::error;
use std::fmt;
use reqwest;
use git2;
use std;

#[derive(Debug)]
pub enum IManError {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    Git(git2::Error),
}

impl fmt::Display for IManError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IManError::Reqwest(ref err) => write!(f, "Error in request: {}", err),
            IManError::Io(ref err) => write!(f, "IO error: {}", err),
            IManError::Git(ref err) => write!(f, "git error: {}", err),
        }
    }
}

impl error::Error for IManError {
    fn description(&self) -> &str {
        match *self {
            IManError::Reqwest(ref err) => err.description(),
            IManError::Io(ref err) => err.description(),
            IManError::Git(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            IManError::Reqwest(ref err) => Some(err),
            IManError::Io(ref err) => Some(err),
            IManError::Git(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for IManError {
    fn from(err: std::io::Error) -> IManError {
        IManError::Io(err)
    }
}

impl From<reqwest::Error> for IManError {
    fn from(err: reqwest::Error) -> IManError {
        IManError::Reqwest(err)
    }
}

impl From<git2::Error> for IManError {
    fn from(err: git2::Error) -> IManError {
        IManError::Git(err)
    }
}