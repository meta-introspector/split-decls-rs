// Generated macro for impl_315 (impl)
macro_rules! Depcrate_provider_lstmimpl_315 {
() => {
// Module: crate::provider::lstm
// Provides: {"impl_315"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl databake :: BakeSize for LstmDataFloat32 < '_ > { fn borrows_size (& self) -> usize { self . model . borrows_size () + self . dic . borrows_size () + self . embedding . borrows_size () + self . fw_w . borrows_size () + self . fw_u . borrows_size () + self . fw_b . borrows_size () + self . bw_w . borrows_size () + self . bw_u . borrows_size () + self . bw_b . borrows_size () + self . time_w . borrows_size () + self . time_b . borrows_size () } }
};
}
