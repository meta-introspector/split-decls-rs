// Generated macro for XmssSig (struct)
macro_rules! Depcrate_xmssXmssSig {
() => {
// Module: crate::xmss
// Provides: {"XmssSig"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq)] pub (crate) struct XmssSig < P : XmssParams > { pub (crate) sig : WotsSig < P > , pub (crate) auth : Array < Array < u8 , P :: N > , P :: HPrime > , }
};
}
