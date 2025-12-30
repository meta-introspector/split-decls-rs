// Generated macro for impl_42 (impl)
macro_rules! Depcrate_forsimpl_42 {
() => {
// Module: crate::fors
// Provides: {"impl_42"}
// Dependencies: {}
impl < P : ForsParams > TryFrom < & [u8] > for ForsMTSig < P > { type Error = () ; fn try_from (slice : & [u8]) -> Result < Self , Self :: Error > { if slice . len () != ForsMTSig :: < P > :: SIZE { return Err (()) ; } # [allow (deprecated)] let sk = Array :: clone_from_slice (& slice [.. P :: N :: USIZE]) ; let mut auth : Array < Array < u8 , P :: N > , P :: A > = Array :: default () ; for i in 0 .. P :: A :: USIZE { auth [i] . copy_from_slice (& slice [P :: N :: USIZE + i * P :: N :: USIZE .. P :: N :: USIZE + (i + 1) * P :: N :: USIZE] ,) ; } Ok (Self { sk , auth }) } }
};
}
