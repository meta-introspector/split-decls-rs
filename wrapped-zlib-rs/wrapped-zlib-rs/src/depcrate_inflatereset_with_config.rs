// Generated macro for reset_with_config (function)
macro_rules! Depcrate_inflatereset_with_config {
() => {
// Module: crate::inflate
// Provides: {"reset_with_config"}
// Dependencies: {}
pub fn reset_with_config (stream : & mut InflateStream , config : InflateConfig) -> ReturnCode { let mut window_bits = config . window_bits ; let wrap ; if window_bits < 0 { wrap = 0 ; if window_bits < - MAX_WBITS { return ReturnCode :: StreamError ; } window_bits = - window_bits ; } else { wrap = (window_bits >> 4) + 5 ; if window_bits < 48 { window_bits &= MAX_WBITS ; } } if window_bits != 0 && ! (MIN_WBITS ..= MAX_WBITS) . contains (& window_bits) { # [cfg (feature = "std")] eprintln ! ("invalid windowBits") ; return ReturnCode :: StreamError ; } if stream . state . window . size () != 0 && stream . state . wbits as i32 != window_bits { let mut window = Window :: empty () ; core :: mem :: swap (& mut window , & mut stream . state . window) ; let (ptr , len) = window . into_raw_parts () ; assert_ne ! (len , 0) ; unsafe { stream . alloc . deallocate (ptr , len) } ; } stream . state . wrap = wrap as u8 ; stream . state . wbits = window_bits as _ ; reset (stream) }
};
}
