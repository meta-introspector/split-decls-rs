// Generated macro for impl_19 (impl)
macro_rules! Depcrate_xsalsaimpl_19 {
() => {
// Module: crate::xsalsa
// Provides: {"impl_19"}
// Dependencies: {}
impl < R : Unsigned > KeyIvInit for XSalsaCore < R > { # [inline] fn new (key : & Key , iv : & XNonce) -> Self { let subkey = hsalsa :: < R > (key , iv [.. 16] . try_into () . unwrap ()) ; let mut padded_iv = Nonce :: default () ; padded_iv . copy_from_slice (& iv [16 ..]) ; XSalsaCore (SalsaCore :: new (& subkey , & padded_iv)) } }
};
}
