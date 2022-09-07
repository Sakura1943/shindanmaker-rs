use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
  #[error("Getting element text failed")]
  GetElementTextErr
}
