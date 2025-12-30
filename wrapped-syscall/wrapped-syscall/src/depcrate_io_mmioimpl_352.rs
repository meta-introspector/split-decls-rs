// Generated macro for impl_352 (impl)
macro_rules! Depcrate_io_mmioimpl_352 {
() => {
// Module: crate::io::mmio
// Provides: {"impl_352"}
// Dependencies: {}
# [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] impl Io for Mmio < u8 > { type Value = u8 ; fn read (& self) -> Self :: Value { unsafe { let value : Self :: Value ; let ptr : * const Self :: Value = ptr :: addr_of ! (self . value) . cast :: < Self :: Value > () ; core :: arch :: asm ! ("mov {}, [{}]" , out (reg_byte) value , in (reg) ptr) ; value } } fn write (& mut self , value : Self :: Value) { unsafe { let ptr : * mut Self :: Value = ptr :: addr_of_mut ! (self . value) . cast :: < Self :: Value > () ; core :: arch :: asm ! ("mov [{}], {}" , in (reg) ptr , in (reg_byte) value ,) ; } } }
};
}
