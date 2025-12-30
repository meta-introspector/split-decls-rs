// Generated macro for impl_158 (impl)
macro_rules! Depcrate_itemimpl_158 {
() => {
// Module: crate::item
// Provides: {"impl_158"}
// Dependencies: {}
# [cfg (feature = "display")] impl std :: fmt :: Display for Item { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match & self { Self :: None => Ok (()) , Self :: Value (v) => v . fmt (f) , Self :: Table (v) => v . fmt (f) , Self :: ArrayOfTables (v) => v . fmt (f) , } } }
};
}
