// Generated macro for impl_168 (impl)
macro_rules! Depcrateimpl_168 {
() => {
// Module: crate
// Provides: {"impl_168"}
// Dependencies: {}
impl TryFrom < usize > for U24 { type Error = Error ; fn try_from (value : usize) -> Result < Self , Self :: Error > { const LEN : usize = core :: mem :: size_of :: < usize > () ; if value > (1 << 24) - 1 { Err (Error :: LibraryError) } else { Ok (U24 (value . to_be_bytes () [LEN - 3 ..] . try_into () ?)) } } }
};
}
