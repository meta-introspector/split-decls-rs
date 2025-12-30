// Generated macro for impl_22 (impl)
macro_rules! Depcrate_biteqimpl_22 {
() => {
// Module: crate::biteq
// Provides: {"impl_22"}
// Dependencies: {}
impl < T : BitEq , const N : usize > BitEq for [T ; N] { fn biteq (& self , other : & Self) -> bool { self . iter () . zip (other . iter ()) . fold (true , | value , (left , right) | value && left . biteq (right)) } fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { # [repr (transparent)] struct Wrapper < 'a , T : BitEq > (& 'a T) ; impl < T : BitEq > core :: fmt :: Debug for Wrapper < '_ , T > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { self . 0 . fmt (f) } } f . debug_list () . entries (self . iter () . map (| x | Wrapper (x))) . finish () } }
};
}
