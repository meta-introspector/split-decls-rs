// Generated macro for impl_582 (impl)
macro_rules! Depcrate_tableimpl_582 {
() => {
// Module: crate::table
// Provides: {"impl_582"}
// Dependencies: {}
# [cfg (feature = "display")] impl core :: fmt :: Display for Table { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { crate :: ser :: to_string (self) . expect ("Unable to represent value as string") . fmt (f) } }
};
}
