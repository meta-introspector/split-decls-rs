// Generated macro for sync_point (function)
macro_rules! Depcrate_inflatesync_point {
() => {
// Module: crate::inflate
// Provides: {"sync_point"}
// Dependencies: {}
pub fn sync_point (stream : & mut InflateStream) -> bool { matches ! (stream . state . mode , Mode :: Stored) && stream . state . bit_reader . bits_in_buffer () == 0 }
};
}
