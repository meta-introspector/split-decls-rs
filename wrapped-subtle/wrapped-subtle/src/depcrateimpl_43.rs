// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl < T > From < CtOption < T > > for Option < T > { # [doc = " Convert the `CtOption<T>` wrapper into an `Option<T>`, depending on whether"] # [doc = " the underlying `is_some` `Choice` was a `0` or a `1` once unwrapped."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This function exists to avoid ending up with ugly, verbose and/or bad handled"] # [doc = " conversions from the `CtOption<T>` wraps to an `Option<T>` or `Result<T, E>`."] # [doc = " This implementation doesn't intend to be constant-time nor try to protect the"] # [doc = " leakage of the `T` since the `Option<T>` will do it anyways."] fn from (source : CtOption < T >) -> Option < T > { if source . is_some () . unwrap_u8 () == 1u8 { Option :: Some (source . value) } else { None } } }
};
}
