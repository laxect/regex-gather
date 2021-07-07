use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("no enough input provided")]
    NoEnoughInput,
}

pub type Result<T> = std::result::Result<T, Error>;
