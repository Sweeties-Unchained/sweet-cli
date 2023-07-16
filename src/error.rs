#[derive(Debug)]
pub enum Error {
    KeyPairGenerationError,
    Unspecified,
}

impl From<ring::error::KeyRejected> for Error {
    fn from(_: ring::error::KeyRejected) -> Self {
        Error::KeyPairGenerationError
    }
}

impl From<ring::error::Unspecified> for Error {
    fn from(_: ring::error::Unspecified) -> Self {
        Error::Unspecified
    }
}
