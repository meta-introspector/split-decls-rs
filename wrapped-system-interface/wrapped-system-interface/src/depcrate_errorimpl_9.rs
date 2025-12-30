// Generated macro for impl_9 (impl)
macro_rules! Depcrate_errorimpl_9 {
() => {
// Module: crate::error
// Provides: {"impl_9"}
// Dependencies: {}
impl ToStr for SystemError { fn to_str (& self) -> & 'static str { match self { SystemError :: AccountAlreadyInUse => "an account with the same address already exists" , SystemError :: ResultWithNegativeLamports => { "account does not have enough SOL to perform the operation" } SystemError :: InvalidProgramId => "cannot assign account to this program id" , SystemError :: InvalidAccountDataLength => "cannot allocate account data of this length" , SystemError :: MaxSeedLengthExceeded => "length of requested seed is too long" , SystemError :: AddressWithSeedMismatch => { "provided address does not match addressed derived from seed" } SystemError :: NonceNoRecentBlockhashes => { "advancing stored nonce requires a populated RecentBlockhashes sysvar" } SystemError :: NonceBlockhashNotExpired => "stored nonce is still in recent_blockhashes" , SystemError :: NonceUnexpectedBlockhashValue => { "specified nonce does not match stored nonce" } } } }
};
}
