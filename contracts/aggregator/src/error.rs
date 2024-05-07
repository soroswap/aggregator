use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AggregatorError {
    NotInitialized = 501,
    AlreadyInitialized = 502,
    NegativeNotAllowed = 503,
    ProtocolAddressNotFound = 504,
    DeadlineExpired = 505,

    InsufficientAAmount = 405,
    InsufficientBAmount = 406,
    InsufficientOutputAmount = 407,
    ExcessiveInputAmount = 408,
    UnsupportedProtocol = 409,
    DistributionLengthExceeded = 417,
    InvalidTotalParts = 418,
    ArithmeticError = 419,
}