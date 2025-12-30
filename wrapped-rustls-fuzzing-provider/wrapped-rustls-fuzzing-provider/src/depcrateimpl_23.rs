// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl crypto :: KeyProvider for Provider { fn load_private_key (& self , _key_der : PrivateKeyDer < 'static > ,) -> Result < Box < dyn crypto :: SigningKey > , Error > { Ok (Box :: new (SigningKey)) } }
};
}
