// Generated macro for private (module)
macro_rules! Depcrateprivate {
() => {
// Module: crate
// Provides: {"private"}
// Dependencies: {}
pub (crate) mod private { pub trait Sealed { } impl Sealed for usize { } impl Sealed for str { } impl Sealed for String { } impl Sealed for i64 { } impl Sealed for f64 { } impl Sealed for bool { } impl Sealed for crate :: Datetime { } impl < T : ? Sized > Sealed for & T where T : Sealed { } impl Sealed for crate :: Table { } impl Sealed for crate :: InlineTable { } }
};
}
