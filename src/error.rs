use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Getting element text failed")]
    GetElementTextErr,
}
