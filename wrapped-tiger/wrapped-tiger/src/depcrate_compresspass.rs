// Generated macro for pass (function)
macro_rules! Depcrate_compresspass {
() => {
// Module: crate::compress
// Provides: {"pass"}
// Dependencies: {}
# [inline (always)] fn pass (a : & mut u64 , b : & mut u64 , c : & mut u64 , x : & [u64 ; 8] , mul : u8) { round (a , b , c , & x [0] , mul) ; round (b , c , a , & x [1] , mul) ; round (c , a , b , & x [2] , mul) ; round (a , b , c , & x [3] , mul) ; round (b , c , a , & x [4] , mul) ; round (c , a , b , & x [5] , mul) ; round (a , b , c , & x [6] , mul) ; round (b , c , a , & x [7] , mul) ; }
};
}
