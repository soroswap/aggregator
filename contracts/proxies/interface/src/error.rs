use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ProxyError {
    NotInitialized = 701,
    AlreadyInitialized = 702,
    NegativeNotAllowed = 703,
    ProtocolAddressNotFound = 704,
    DeadlineExpired = 705,
}

