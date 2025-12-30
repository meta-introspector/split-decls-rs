// Generated macro for impl_179 (impl)
macro_rules! Depcrate_serial_numberimpl_179 {
() => {
// Module: crate::serial_number
// Provides: {"impl_179"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < 'a , P : Profile > arbitrary :: Arbitrary < 'a > for SerialNumber < P > { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { let len = u . int_in_range (0u32 ..= Self :: MAX_LEN . into ()) ? ; Self :: new (u . bytes (len as usize) ?) . map_err (| _ | arbitrary :: Error :: IncorrectFormat) } fn size_hint (depth : usize) -> (usize , Option < usize >) { arbitrary :: size_hint :: and (u32 :: size_hint (depth) , (0 , None)) } }
};
}
