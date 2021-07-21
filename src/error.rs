/// An error related to Ed25519 signatures.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    /// The encoding of a secret key was malformed.
    MalformedSecretKey,
    /// The encoding of a public key was malformed.
    MalformedPublicKey,
    /// Signature verification failed.
    InvalidSignature,
    /// A byte slice of the wrong length was supplied during parsing.
    InvalidSliceLength,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::MalformedSecretKey => write!(f, "Malformed secret key encoding."),
            Error::MalformedPublicKey => write!(f, "Malformed public key encoding."),
            Error::InvalidSignature => write!(f, "Invalid signature."),
            Error::InvalidSliceLength => write!(f, "Invalid length when parsing byte slice."),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
