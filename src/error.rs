use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Edge not found")]
    EdgeNotFound,
    #[error("Connection already exists")]
    EdgeAlreadyExists,
}
