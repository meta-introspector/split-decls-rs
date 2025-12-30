// Generated macro for try_set_output_capture (function)
macro_rules! Depcrate_io_stdiotry_set_output_capture {
() => {
// Module: crate::io::stdio
// Provides: {"try_set_output_capture"}
// Dependencies: {}
# [doc = " Tries to set the thread-local output capture buffer and returns the old one."] # [doc = " This may fail once thread-local destructors are called. It's used in panic"] # [doc = " handling instead of `set_output_capture`."] # [unstable (feature = "internal_output_capture" , reason = "this function is meant for use in the test crate \
    and may disappear in the future" , issue = "none")] # [doc (hidden)] pub fn try_set_output_capture (sink : Option < LocalStream > ,) -> Result < Option < LocalStream > , AccessError > { if sink . is_none () && ! OUTPUT_CAPTURE_USED . load (Ordering :: Relaxed) { return Ok (None) ; } OUTPUT_CAPTURE_USED . store (true , Ordering :: Relaxed) ; OUTPUT_CAPTURE . try_with (move | slot | slot . replace (sink)) }
};
}
