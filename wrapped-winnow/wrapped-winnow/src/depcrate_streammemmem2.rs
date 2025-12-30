// Generated macro for memmem2 (function)
macro_rules! Depcrate_streammemmem2 {
() => {
// Module: crate::stream
// Provides: {"memmem2"}
// Dependencies: {}
# [inline (always)] fn memmem2 (slice : & [u8] , literal : (& [u8] , & [u8])) -> Option < core :: ops :: Range < usize > > { match (literal . 0 . len () , literal . 1 . len ()) { (0 , _) | (_ , 0) => Some (0 .. 0) , (1 , 1) => memchr2 ((literal . 0 [0] , literal . 1 [0]) , slice) . map (| i | i .. i + 1) , _ => memmem2_ (slice , literal) , } }
};
}
