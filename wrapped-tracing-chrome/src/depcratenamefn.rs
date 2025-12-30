// Generated macro for NameFn (type)
macro_rules! DepcrateNameFn {
() => {
// Module: crate
// Provides: {"NameFn"}
// Dependencies: {}
type NameFn < S > = Box < dyn Fn (& EventOrSpan < '_ , '_ , S >) -> String + Send + Sync > ;
};
}
