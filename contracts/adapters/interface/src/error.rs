use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AdapterError {
    NotInitialized = 401,
    AlreadyInitialized = 402,
    NegativeNotAllowed = 403,
    ProtocolAddressNotFound = 404,
    DeadlineExpired = 405,
    MissingPoolHashes = 406, // For AQUA
    WrongMinimumPathLength = 407,
    WrongPoolHashesLength = 408, // For AQUA
}

