// Generated macro for impl_361 (impl)
macro_rules! Depcrate_io_pioimpl_361 {
() => {
// Module: crate::io::pio
// Provides: {"impl_361"}
// Dependencies: {}
# [doc = " Read/Write for byte PIO"] impl Io for Pio < u8 > { type Value = u8 ; # [doc = " Read"] # [inline (always)] fn read (& self) -> u8 { let value : u8 ; unsafe { asm ! ("in al, dx" , in ("dx") self . port , out ("al") value , options (nostack , nomem , preserves_flags)) ; } value } # [doc = " Write"] # [inline (always)] fn write (& mut self , value : u8) { unsafe { asm ! ("out dx, al" , in ("dx") self . port , in ("al") value , options (nostack , nomem , preserves_flags)) ; } } }
};
}
