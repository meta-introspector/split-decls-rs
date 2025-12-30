// Generated macro for impl_22 (impl)
macro_rules! Depcrate_sipimpl_22 {
() => {
// Module: crate::sip
// Provides: {"impl_22"}
// Dependencies: {}
impl < S : Sip > Hasher < S > { # [inline] fn new_with_keys (key0 : u64 , key1 : u64) -> Hasher < S > { let mut state = Hasher { k0 : key0 , k1 : key1 , length : 0 , state : State { v0 : 0 , v1 : 0 , v2 : 0 , v3 : 0 , } , tail : 0 , ntail : 0 , _marker : PhantomData , } ; state . reset () ; state } # [inline] fn reset (& mut self) { self . length = 0 ; self . state . v0 = self . k0 ^ 0x736f6d6570736575 ; self . state . v1 = self . k1 ^ 0x646f72616e646f6d ; self . state . v2 = self . k0 ^ 0x6c7967656e657261 ; self . state . v3 = self . k1 ^ 0x7465646279746573 ; self . ntail = 0 ; } # [inline] fn short_write < T > (& mut self , _x : T , x : u64) { let size = mem :: size_of :: < T > () ; self . length += size ; debug_assert ! (if size < 8 { x >> (8 * size) == 0 } else { true }) ; let needed = 8 - self . ntail ; self . tail |= x << (8 * self . ntail) ; if size < needed { self . ntail += size ; return ; } self . state . v3 ^= self . tail ; S :: c_rounds (& mut self . state) ; self . state . v0 ^= self . tail ; self . ntail = size - needed ; self . tail = if needed < 8 { x >> (8 * needed) } else { 0 } ; } }
};
}
