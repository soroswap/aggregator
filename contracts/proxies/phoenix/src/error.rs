use soroban_sdk::{self, contracterror};
use soroswap_library::{SoroswapLibraryError};


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SoroswapAggregatorProxyError {
    /// SoroswapAggregatorProxy: not yet initialized
    NotInitialized = 401,

    /// SoroswapAggregatorProxy: negative amount is not allowed
    NegativeNotAllowed = 402,

    /// SoroswapAggregatorProxy: deadline expired
    DeadlineExpired = 403,
    
    /// SoroswapAggregatorProxy: already initialized
    InitializeAlreadyInitialized = 404,

    /// SoroswapAggregatorProxy: insufficient a amount
    InsufficientAmount = 405,

    /// SoroswapAggregatorProxy: Error swapping
    SwapError = 406,

    /// SoroswapAggregatorProxy: insufficient output amount
    InsufficientOutputAmount = 407,

    /// SoroswapAggregatorProxy: is Paused
    ContractPaused = 408,

    /// SoroswapAggregatorProxy: Protocol address not found
    ProtocolAddressNotFound = 416,
}


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
// Define a new set of integer literals for the CombinedError enum
pub enum CombinedProxyError {
    ProxyNotInitialized = 501,
    ProxyNegativeNotAllowed = 502,
    ProxyDeadlineExpired = 503,
    ProxyInitializeAlreadyInitialized = 504,
    ProxyInsufficientAmount = 505,
    ProxySwapError=506,
    ProxyInsufficientOutputAmount = 507,
    ProxyContractPaused = 508,
    ProxyProtocolAddressNotFound = 516,

    LibraryInsufficientAmount = 510,
    LibraryInsufficientLiquidity = 511,
    LibraryInsufficientInputAmount = 512,
    LibraryInsufficientOutputAmount = 513,
    LibraryInvalidPath = 514,
    LibrarySortIdenticalTokens = 515,
}

impl From<SoroswapLibraryError> for CombinedProxyError {
    fn from(err: SoroswapLibraryError) -> Self {
        match err {
            SoroswapLibraryError::InsufficientAmount => CombinedProxyError::LibraryInsufficientAmount,
            SoroswapLibraryError::InsufficientLiquidity => CombinedProxyError::LibraryInsufficientLiquidity,
            SoroswapLibraryError::InsufficientInputAmount => CombinedProxyError::LibraryInsufficientInputAmount,
            SoroswapLibraryError::InsufficientOutputAmount => CombinedProxyError::LibraryInsufficientOutputAmount,
            SoroswapLibraryError::InvalidPath => CombinedProxyError::LibraryInvalidPath,
            SoroswapLibraryError::SortIdenticalTokens => CombinedProxyError::LibrarySortIdenticalTokens,
        }
    }
}

impl From<SoroswapAggregatorProxyError> for CombinedProxyError {
    fn from(err: SoroswapAggregatorProxyError) -> Self {
        match err {
            SoroswapAggregatorProxyError::NotInitialized => CombinedProxyError::ProxyNotInitialized,
            SoroswapAggregatorProxyError::NegativeNotAllowed => CombinedProxyError::ProxyNegativeNotAllowed,
            SoroswapAggregatorProxyError::DeadlineExpired => CombinedProxyError::ProxyDeadlineExpired,
            SoroswapAggregatorProxyError::InitializeAlreadyInitialized => CombinedProxyError::ProxyInitializeAlreadyInitialized,
            SoroswapAggregatorProxyError::InsufficientAmount => CombinedProxyError::ProxyInsufficientAmount,
            SoroswapAggregatorProxyError::SwapError => CombinedProxyError::ProxySwapError,
            SoroswapAggregatorProxyError::InsufficientOutputAmount => CombinedProxyError::ProxyInsufficientOutputAmount,
            SoroswapAggregatorProxyError::ContractPaused => CombinedProxyError::ProxyContractPaused,
            SoroswapAggregatorProxyError::ProtocolAddressNotFound => CombinedProxyError::ProxyProtocolAddressNotFound,
        }
    }
}
