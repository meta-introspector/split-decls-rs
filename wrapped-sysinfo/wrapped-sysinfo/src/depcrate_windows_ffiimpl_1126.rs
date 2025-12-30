// Generated macro for impl_1126 (impl)
macro_rules! Depcrate_windows_ffiimpl_1126 {
() => {
// Module: crate::windows::ffi
// Provides: {"impl_1126"}
// Dependencies: {}
impl std :: fmt :: Display for SMBIOSUuid { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}" , u32 :: from_le (self . time_low) , u16 :: from_le (self . time_mid) , u16 :: from_le (self . time_hi_and_version) , self . clock_seq_hi_and_reserved , self . clock_seq_low , self . node [0] , self . node [1] , self . node [2] , self . node [3] , self . node [4] , self . node [5] ,) } }
};
}
