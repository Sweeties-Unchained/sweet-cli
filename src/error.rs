#[derive(Debug)]
pub enum Error {
    KeyRejected,
    Unspecified,
    ZboxError(zbox::Error),
}

impl From<ring::error::KeyRejected> for Error {
    fn from(_: ring::error::KeyRejected) -> Self {
        Error::KeyRejected
    }
}

impl From<ring::error::Unspecified> for Error {
    fn from(_: ring::error::Unspecified) -> Self {
        Error::Unspecified
    }
}

impl From<zbox::Error> for Error {
    fn from(error: zbox::Error) -> Self {
        Error::ZboxError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::Unspecified
    }
}
