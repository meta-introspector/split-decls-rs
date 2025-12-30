// Generated macro for impl_714 (impl)
macro_rules! Depcrate_rejectimpl_714 {
() => {
// Module: crate::reject
// Provides: {"impl_714"}
// Dependencies: {}
impl fmt :: Debug for Reason { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Reason :: NotFound => f . write_str ("NotFound") , Reason :: Other (ref other) => match * * other { Rejections :: Known (ref e) => fmt :: Debug :: fmt (e , f) , Rejections :: Custom (ref e) => fmt :: Debug :: fmt (e , f) , Rejections :: Combined (ref a , ref b) => { let mut list = f . debug_list () ; a . debug_list (& mut list) ; b . debug_list (& mut list) ; list . finish () } } , } } }
};
}
