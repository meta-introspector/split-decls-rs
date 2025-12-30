// Generated macro for impl_159 (impl)
macro_rules! Depcrate_tendrilimpl_159 {
() => {
// Module: crate::tendril
// Provides: {"impl_159"}
// Dependencies: {}
impl < F , A > Tendril < F , A > where A : Atomicity , F : fmt :: SliceFormat < Slice = [u8] > { # [doc = " Decode from some character encoding into UTF-8."] # [doc = ""] # [doc = " See the [rust-encoding docs](https://lifthrasiir.github.io/rust-encoding/encoding/)"] # [doc = " for more information."] # [inline] pub fn decode (& self , encoding : EncodingRef , trap : DecoderTrap) -> Result < Tendril < fmt :: UTF8 , A > , Cow < 'static , str > > { let mut ret = Tendril :: new () ; encoding . decode_to (& * self , trap , & mut ret) . map (| _ | ret) } # [doc = " Push \"uninitialized bytes\" onto the end."] # [doc = ""] # [doc = " Really, this grows the tendril without writing anything to the new area."] # [doc = " It's only defined for byte tendrils because it's only useful if you"] # [doc = " plan to then mutate the buffer."] # [inline] pub unsafe fn push_uninitialized (& mut self , n : u32) { let new_len = self . len32 () . checked_add (n) . expect (OFLOW) ; if new_len <= MAX_INLINE_LEN as u32 && self . ptr . get () . get () <= MAX_INLINE_TAG { self . ptr . set (inline_tag (new_len)) } else { self . make_owned_with_capacity (new_len) ; self . len = new_len ; } } }
};
}
