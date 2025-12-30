// Generated macro for impl_214 (impl)
macro_rules! Depcrate_xmssimpl_214 {
() => {
// Module: crate::xmss
// Provides: {"impl_214"}
// Dependencies: {}
impl < P : XmssParams > TryFrom < & [u8] > for XmssSig < P > { type Error = () ; fn try_from (value : & [u8]) -> Result < Self , Self :: Error > { if value . len () != Self :: SIZE { return Err (()) ; } let sig = WotsSig :: < P > :: try_from (& value [.. WotsSig :: < P > :: SIZE]) ? ; let mut auth = Array :: < Array < u8 , P :: N > , P :: HPrime > :: default () ; for i in 0 .. P :: HPrime :: USIZE { auth [i] . copy_from_slice (& value [WotsSig :: < P > :: SIZE + i * P :: N :: USIZE .. WotsSig :: < P > :: SIZE + (i + 1) * P :: N :: USIZE] ,) ; } Ok (XmssSig { sig , auth }) } }
};
}
