use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AggregatorError {
    NotInitialized = 601,
    AlreadyInitialized = 602,
    NegativeNotAllowed = 603,
    ProtocolNotFound = 604,
    DeadlineExpired = 605,
    
    // checks of the desired amount.
    // if at the end, the total output is too low (insufficient) for what we expected as amount_out_min
    InsufficientOutputAmount = 608, // the amount of output tokens to receive is insufficient given the provided amount_out_min

    // checks the maximum amount we are willing to spend
    // if at the end, the total amount of paid tokens is too high (excessive) for what we expected as amount_in_max
    ExcessiveInputAmount = 609, // the amount of input tokens required is excessive given the provided amount_in
    
    ProtocolPaused = 610,
    DistributionLengthExceeded = 611,
    ZeroDistributionPart = 612,
    ArithmeticError = 613,
    Unauthorized = 614,
    InvalidPath = 615,

    NegibleAmount = 616,
}
