// Generated macro for impl_17 (impl)
macro_rules! Depcrate_datetimeimpl_17 {
() => {
// Module: crate::datetime
// Provides: {"impl_17"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl fmt :: Display for Datetime { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (ref date) = self . date { write ! (f , "{date}") ? ; } if let Some (ref time) = self . time { if self . date . is_some () { write ! (f , "T") ? ; } write ! (f , "{time}") ? ; } if let Some (ref offset) = self . offset { write ! (f , "{offset}") ? ; } Ok (()) } }
};
}
