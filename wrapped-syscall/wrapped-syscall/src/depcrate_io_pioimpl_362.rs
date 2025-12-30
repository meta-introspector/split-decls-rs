// Generated macro for impl_362 (impl)
macro_rules! Depcrate_io_pioimpl_362 {
() => {
// Module: crate::io::pio
// Provides: {"impl_362"}
// Dependencies: {}
# [doc = " Read/Write for word PIO"] impl Io for Pio < u16 > { type Value = u16 ; # [doc = " Read"] # [inline (always)] fn read (& self) -> u16 { let value : u16 ; unsafe { asm ! ("in ax, dx" , in ("dx") self . port , out ("ax") value , options (nostack , nomem , preserves_flags)) ; } value } # [doc = " Write"] # [inline (always)] fn write (& mut self , value : u16) { unsafe { asm ! ("out dx, ax" , in ("dx") self . port , in ("ax") value , options (nostack , nomem , preserves_flags)) ; } } }
};
}
