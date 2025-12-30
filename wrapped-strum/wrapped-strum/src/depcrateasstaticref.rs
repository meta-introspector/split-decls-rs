// Generated macro for AsStaticRef (trait)
macro_rules! DepcrateAsStaticRef {
() => {
// Module: crate
// Provides: {"AsStaticRef"}
// Dependencies: {}
# [doc = " A cheap reference-to-reference conversion. Used to convert a value to a"] # [doc = " reference value with `'static` lifetime within generic code."] # [deprecated (since = "0.22.0" , note = "please use `#[derive(IntoStaticStr)]` instead")] pub trait AsStaticRef < T > where T : ? Sized , { fn as_static (& self) -> & 'static T ; }
};
}
