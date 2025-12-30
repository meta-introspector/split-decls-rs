// Generated macro for impl_67 (impl)
macro_rules! Depcrate_arrayvecimpl_67 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_67"}
// Dependencies: {}
# [cfg (feature = "experimental_write_impl")] impl < A : Array < Item = u8 > > core :: fmt :: Write for ArrayVec < A > { fn write_str (& mut self , s : & str) -> core :: fmt :: Result { let my_len = self . len () ; let str_len = s . as_bytes () . len () ; if my_len + str_len <= A :: CAPACITY { let remainder = & mut self . data . as_slice_mut () [my_len ..] ; let target = & mut remainder [.. str_len] ; target . copy_from_slice (s . as_bytes ()) ; Ok (()) } else { Err (core :: fmt :: Error) } } }
};
}
