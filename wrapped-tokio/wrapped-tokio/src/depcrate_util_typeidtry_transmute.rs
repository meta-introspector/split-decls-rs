// Generated macro for try_transmute (function)
macro_rules! Depcrate_util_typeidtry_transmute {
() => {
// Module: crate::util::typeid
// Provides: {"try_transmute"}
// Dependencies: {}
pub (super) unsafe fn try_transmute < Src , Target : 'static > (x : Src) -> Result < Target , Src > { if nonstatic_typeid :: < Src > () == TypeId :: of :: < Target > () { let x = ManuallyDrop :: new (x) ; Ok (unsafe { mem :: transmute_copy :: < Src , Target > (& x) }) } else { Err (x) } }
};
}
