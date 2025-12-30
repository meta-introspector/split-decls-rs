// Generated macro for array_vec (macro)
macro_rules! Depcrate_arrayvecarray_vec {
() => {
// Module: crate::arrayvec
// Provides: {"array_vec"}
// Dependencies: {}
# [doc = " Helper to make an `ArrayVec`."] # [doc = ""] # [doc = " You specify the backing array type, and optionally give all the elements you"] # [doc = " want to initially place into the array."] # [doc = ""] # [doc = " ```rust"] # [doc = " use tinyvec::*;"] # [doc = ""] # [doc = " // The backing array type can be specified in the macro call"] # [doc = " let empty_av = array_vec!([u8; 16]);"] # [doc = " let some_ints = array_vec!([i32; 4] => 1, 2, 3);"] # [doc = ""] # [doc = " // Or left to inference"] # [doc = " let empty_av: ArrayVec<[u8; 10]> = array_vec!();"] # [doc = " let some_ints: ArrayVec<[u8; 10]> = array_vec!(5, 6, 7, 8);"] # [doc = " ```"] # [macro_export] macro_rules ! array_vec { ($ array_type : ty => $ ($ elem : expr) ,* $ (,) ?) => { { let mut av : $ crate :: ArrayVec <$ array_type > = Default :: default () ; $ (av . push ($ elem) ;) * av } } ; ($ array_type : ty) => { $ crate :: ArrayVec ::<$ array_type >:: default () } ; ($ ($ elem : expr) ,*) => { $ crate :: array_vec ! (_ => $ ($ elem) ,*) } ; ($ elem : expr ; $ n : expr) => { $ crate :: ArrayVec :: from ([$ elem ; $ n]) } ; () => { $ crate :: array_vec ! (_) } ; }
};
}
