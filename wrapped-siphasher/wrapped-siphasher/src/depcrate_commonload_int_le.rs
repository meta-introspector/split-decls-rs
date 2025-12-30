// Generated macro for load_int_le (macro)
macro_rules! Depcrate_commonload_int_le {
() => {
// Module: crate::common
// Provides: {"load_int_le"}
// Dependencies: {}
# [doc = " Loads an integer of the desired type from a byte stream, in LE order."] macro_rules ! load_int_le { ($ buf : expr , $ i : expr , $ int_ty : ident) => { { debug_assert ! ($ i + :: core :: mem :: size_of ::<$ int_ty > () <= $ buf . len ()) ; let mut data = 0 as $ int_ty ; :: core :: ptr :: copy_nonoverlapping ($ buf . as_ptr () . add ($ i) , & mut data as * mut _ as * mut u8 , :: core :: mem :: size_of ::<$ int_ty > () ,) ; data . to_le () } } ; }
};
}
