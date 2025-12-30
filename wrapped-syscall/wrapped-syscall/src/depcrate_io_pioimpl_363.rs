// Generated macro for impl_363 (impl)
macro_rules! Depcrate_io_pioimpl_363 {
() => {
// Module: crate::io::pio
// Provides: {"impl_363"}
// Dependencies: {}
# [doc = " Read/Write for doubleword PIO"] impl Io for Pio < u32 > { type Value = u32 ; # [doc = " Read"] # [inline (always)] fn read (& self) -> u32 { let value : u32 ; unsafe { asm ! ("in eax, dx" , in ("dx") self . port , out ("eax") value , options (nostack , nomem , preserves_flags)) ; } value } # [doc = " Write"] # [inline (always)] fn write (& mut self , value : u32) { unsafe { asm ! ("out dx, eax" , in ("dx") self . port , in ("eax") value , options (nostack , nomem , preserves_flags)) ; } } }
};
}
