// Generated macro for impl_38 (impl)
macro_rules! Depcrate_approxeqimpl_38 {
() => {
// Module: crate::approxeq
// Provides: {"impl_38"}
// Dependencies: {}
impl < T : ApproxEq , const N : usize > ApproxEq for [T ; N] { fn approxeq (& self , other : & Self , ulps : i64) -> bool { self . iter () . zip (other . iter ()) . fold (true , | value , (left , right) | { value && left . approxeq (right , ulps) }) } fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { # [repr (transparent)] struct Wrapper < 'a , T : ApproxEq > (& 'a T) ; impl < T : ApproxEq > core :: fmt :: Debug for Wrapper < '_ , T > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { self . 0 . fmt (f) } } f . debug_list () . entries (self . iter () . map (| x | Wrapper (x))) . finish () } }
};
}
