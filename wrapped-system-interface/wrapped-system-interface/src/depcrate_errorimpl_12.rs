// Generated macro for impl_12 (impl)
macro_rules! Depcrate_errorimpl_12 {
() => {
// Module: crate::error
// Provides: {"impl_12"}
// Dependencies: {}
impl TryFrom < u32 > for SystemError { type Error = ProgramError ; fn try_from (error : u32) -> Result < Self , Self :: Error > { match error { 0 => Ok (SystemError :: AccountAlreadyInUse) , 1 => Ok (SystemError :: ResultWithNegativeLamports) , 2 => Ok (SystemError :: InvalidProgramId) , 3 => Ok (SystemError :: InvalidAccountDataLength) , 4 => Ok (SystemError :: MaxSeedLengthExceeded) , 5 => Ok (SystemError :: AddressWithSeedMismatch) , 6 => Ok (SystemError :: NonceNoRecentBlockhashes) , 7 => Ok (SystemError :: NonceBlockhashNotExpired) , 8 => Ok (SystemError :: NonceUnexpectedBlockhashValue) , _ => Err (ProgramError :: InvalidArgument) , } } }
};
}
