// Generated macro for sealed (module)
macro_rules! Depcrate_convertsealed {
() => {
// Module: crate::convert
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { # [doc = " A trait for defining the ratio of two units of time."] # [doc = ""] # [doc = " This trait is used to implement the `per` method on the various structs."] # [diagnostic :: on_unimplemented (message = "`{Self}` is not an integer multiple of `{T}`")] pub trait MultipleOf < T , Output > { # [doc = " The number of one unit of time in the other."] const VALUE : Output ; } pub trait DefaultOutput < T > { type Output ; } }
};
}
