// Generated macro for impl_82 (impl)
macro_rules! Depcrate_buf32impl_82 {
() => {
// Module: crate::buf32
// Provides: {"impl_82"}
// Dependencies: {}
impl < H > Buf32 < H > { # [inline] pub unsafe fn with_capacity (mut cap : u32 , h : H) -> Buf32 < H > { if cap < MIN_CAP { cap = MIN_CAP ; } let mut vec = Vec :: < H > :: with_capacity (bytes_to_vec_capacity :: < H > (cap)) ; let ptr = vec . as_mut_ptr () ; mem :: forget (vec) ; ptr :: write (ptr , h) ; Buf32 { ptr : ptr , len : 0 , cap : cap , } } # [inline] pub unsafe fn destroy (self) { mem :: drop (Vec :: from_raw_parts (self . ptr , 1 , bytes_to_vec_capacity :: < H > (self . cap))) ; } # [inline (always)] pub unsafe fn data_ptr (& self) -> * mut u8 { (self . ptr as * mut u8) . offset (mem :: size_of :: < H > () as isize) } # [inline (always)] pub unsafe fn data (& self) -> & [u8] { slice :: from_raw_parts (self . data_ptr () , self . len as usize) } # [inline (always)] pub unsafe fn data_mut (& mut self) -> & mut [u8] { slice :: from_raw_parts_mut (self . data_ptr () , self . len as usize) } # [doc = " Grow the capacity to at least `new_cap`."] # [doc = ""] # [doc = " This will panic if the capacity calculation overflows `u32`."] # [inline] pub unsafe fn grow (& mut self , new_cap : u32) { if new_cap <= self . cap { return ; } let new_cap = new_cap . checked_next_power_of_two () . expect (OFLOW) ; let mut vec = Vec :: from_raw_parts (self . ptr , 0 , bytes_to_vec_capacity :: < H > (self . cap)) ; vec . reserve_exact (bytes_to_vec_capacity :: < H > (new_cap)) ; self . ptr = vec . as_mut_ptr () ; self . cap = new_cap ; mem :: forget (vec) ; } }
};
}
