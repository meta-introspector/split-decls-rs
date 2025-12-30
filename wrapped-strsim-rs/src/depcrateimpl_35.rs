// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl < ValueType > HybridGrowingHashmapChar < ValueType > where ValueType : Default + Clone + Copy + Eq , { fn get (& self , key : char) -> ValueType { let value = key as u32 ; if value <= 255 { let val_u8 = u8 :: try_from (value) . expect ("we check the bounds above") ; self . extended_ascii [usize :: from (val_u8)] } else { self . map . get (value) } } fn get_mut (& mut self , key : char) -> & mut ValueType { let value = key as u32 ; if value <= 255 { let val_u8 = u8 :: try_from (value) . expect ("we check the bounds above") ; & mut self . extended_ascii [usize :: from (val_u8)] } else { self . map . get_mut (value) } } }
};
}
