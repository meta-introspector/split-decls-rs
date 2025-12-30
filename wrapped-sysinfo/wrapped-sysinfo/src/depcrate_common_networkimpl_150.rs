// Generated macro for impl_150 (impl)
macro_rules! Depcrate_common_networkimpl_150 {
() => {
// Module: crate::common::network
// Provides: {"impl_150"}
// Dependencies: {}
impl fmt :: Display for MacAddr { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let data = & self . 0 ; write ! (f , "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}" , data [0] , data [1] , data [2] , data [3] , data [4] , data [5] ,) } }
};
}
