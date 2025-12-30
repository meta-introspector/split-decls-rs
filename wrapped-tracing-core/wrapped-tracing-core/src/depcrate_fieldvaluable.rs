// Generated macro for valuable (function)
macro_rules! Depcrate_fieldvaluable {
() => {
// Module: crate::field
// Provides: {"valuable"}
// Dependencies: {}
# [doc = " Wraps a type implementing [`Valuable`] as a `Value` that"] # [doc = " can be recorded using its `Valuable` implementation."] # [doc = ""] # [doc = " [`Valuable`]: https://docs.rs/valuable/latest/valuable/trait.Valuable.html"] # [cfg (all (tracing_unstable , feature = "valuable"))] # [cfg_attr (docsrs , doc (cfg (all (tracing_unstable , feature = "valuable"))))] pub fn valuable < T > (t : & T) -> valuable :: Value < '_ > where T : valuable :: Valuable , { t . as_value () }
};
}
