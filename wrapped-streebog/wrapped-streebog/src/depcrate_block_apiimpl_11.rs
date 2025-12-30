// Generated macro for impl_11 (impl)
macro_rules! Depcrate_block_apiimpl_11 {
() => {
// Module: crate::block_api
// Provides: {"impl_11"}
// Dependencies: {}
impl StreebogVarCore { # [inline (always)] fn update_sigma (& mut self , m : & [u64 ; 8]) { let mut carry = false ; # [allow (clippy :: needless_range_loop)] for i in 0 .. 8 { adc (& mut self . sigma [i] , m [i] , & mut carry) ; } } # [inline (always)] fn update_n (& mut self , len : u64) { let mut carry = false ; adc (& mut self . n [0] , 8 * len , & mut carry) ; for i in 1 .. 7 { adc (& mut self . n [i] , 0 , & mut carry) ; } } # [inline (always)] fn compress (& mut self , block : & [u8 ; 64] , msg_len : u64) { let block = from_bytes (block) ; g (& mut self . h , & self . n , & block) ; self . update_n (msg_len) ; self . update_sigma (& block) ; } }
};
}
