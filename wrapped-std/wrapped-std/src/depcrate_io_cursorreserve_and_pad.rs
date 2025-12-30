// Generated macro for reserve_and_pad (function)
macro_rules! Depcrate_io_cursorreserve_and_pad {
() => {
// Module: crate::io::cursor
// Provides: {"reserve_and_pad"}
// Dependencies: {}
# [doc = " Reserves the required space, and pads the vec with 0s if necessary."] fn reserve_and_pad < A : Allocator > (pos_mut : & mut u64 , vec : & mut Vec < u8 , A > , buf_len : usize ,) -> io :: Result < usize > { let pos : usize = (* pos_mut) . try_into () . map_err (| _ | { io :: const_error ! (ErrorKind :: InvalidInput , "cursor position exceeds maximum possible vector length" ,) }) ? ; let desired_cap = pos . saturating_add (buf_len) ; if desired_cap > vec . capacity () { vec . reserve (desired_cap - vec . len ()) ; } if pos > vec . len () { let diff = pos - vec . len () ; let spare = vec . spare_capacity_mut () ; debug_assert ! (spare . len () >= diff) ; unsafe { spare . get_unchecked_mut (.. diff) . fill (core :: mem :: MaybeUninit :: new (0)) ; vec . set_len (pos) ; } } Ok (pos) }
};
}
