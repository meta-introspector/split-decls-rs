// Generated macro for impl_232 (impl)
macro_rules! Depcrate_de_implsimpl_232 {
() => {
// Module: crate::de::impls
// Provides: {"impl_232"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (fake_variadic))] # [cfg_attr (docsrs , doc = "This trait is implemented for tuples up to 16 items long.")] impl < 'de , T > Deserialize < 'de > for (T ,) where T : Deserialize < 'de > , { tuple_impl_body ! (1 => (0 T)) ; }
};
}
