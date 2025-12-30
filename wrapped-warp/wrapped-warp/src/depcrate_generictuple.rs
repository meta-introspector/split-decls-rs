// Generated macro for Tuple (trait)
macro_rules! Depcrate_genericTuple {
() => {
// Module: crate::generic
// Provides: {"Tuple"}
// Dependencies: {}
pub trait Tuple : Sized { type HList : HList < Tuple = Self > ; fn hlist (self) -> Self :: HList ; # [inline] fn combine < T > (self , other : T) -> CombinedTuples < Self , T > where Self : Sized , T : Tuple , Self :: HList : Combine < T :: HList > , { self . hlist () . combine (other . hlist ()) . flatten () } }
};
}
