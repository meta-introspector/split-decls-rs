// Generated macro for private (module)
macro_rules! Depcrate_fieldprivate {
() => {
// Module: crate::field
// Provides: {"private"}
// Dependencies: {}
mod private { use super :: * ; # [doc = " Restrictions on `ValueSet` lengths were removed in #2508 but this type remains for backwards compatibility."] pub trait ValidLen < 'a > : Borrow < [(& 'a Field , Option < & 'a (dyn Value + 'a) >)] > { } impl < 'a , const N : usize > ValidLen < 'a > for [(& 'a Field , Option < & 'a (dyn Value + 'a) >) ; N] { } }
};
}
