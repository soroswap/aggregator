use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
<<<<<<< HEAD:contracts/proxies/interface/src/error.rs
pub enum ProxyError {
    NotInitialized = 701,
    AlreadyInitialized = 702,
    NegativeNotAllowed = 703,
    ProtocolAddressNotFound = 704,
    DeadlineExpired = 705,
=======
pub enum AdapterError {
    NotInitialized = 401,
    AlreadyInitialized = 402,
    NegativeNotAllowed = 403,
    ProtocolAddressNotFound = 404,
    DeadlineExpired = 405,
>>>>>>> main:contracts/adapters/interface/src/error.rs
}

