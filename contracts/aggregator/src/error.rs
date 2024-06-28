use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AggregatorError {
    NotInitialized = 601,
    AlreadyInitialized = 602,
    NegativeNotAllowed = 603,
    ProtocolAddressNotFound = 604,
    DeadlineExpired = 605,
    InsufficientAAmount = 606,
    InsufficientBAmount = 607,
    InsufficientOutputAmount = 608,
    ExcessiveInputAmount = 609,
    UnsupportedProtocol = 610,
    DistributionLengthExceeded = 611,
    InvalidTotalParts = 612,
    ArithmeticError = 613,
}