// Generated macro for impl_23 (impl)
macro_rules! Depcrate_clausesimpl_23 {
() => {
// Module: crate::clauses
// Provides: {"impl_23"}
// Dependencies: {}
impl ClauseLits { # [doc = " Create a new ClauseLits, storing them in the given buffer if necessary"] fn new (lits : & [Lit] , buffer : & mut Vec < Lit >) -> ClauseLits { let mut inline = [0 ; INLINE_LITS] ; let length = lits . len () ; if length > INLINE_LITS { inline [0] = buffer . len () . try_into () . expect ("exceeded maximal literal buffer size") ; buffer . extend (lits) ; } else { let lits = unsafe { # [allow (clippy :: transmute_ptr_to_ptr)] transmute :: < & [Lit] , & [LitIdx] > (lits) } ; inline [.. length] . copy_from_slice (lits) ; } ClauseLits { length : length as LitIdx , inline , } } # [doc = " Returns the literals as a slice given a storage buffer"] pub fn slice < 'a , 'b , 'c > (& 'a self , buffer : & 'b [Lit]) -> & 'c [Lit] where 'a : 'c , 'b : 'c , { if self . length > INLINE_LITS as LitIdx { & buffer [self . inline [0] as usize ..] [.. self . length as usize] } else { unsafe { # [allow (clippy :: transmute_ptr_to_ptr)] transmute :: < & [LitIdx] , & [Lit] > (& self . inline [.. self . length as usize]) } } } # [doc = " Literals stored in the literal buffer"] fn buffer_used (& self) -> usize { if self . length > INLINE_LITS as LitIdx { self . length as usize } else { 0 } } }
};
}
