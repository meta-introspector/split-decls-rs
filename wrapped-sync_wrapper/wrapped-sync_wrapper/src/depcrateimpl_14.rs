// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
# [cfg (feature = "futures")] impl < S : futures_core :: Stream > futures_core :: Stream for SyncStream < S > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let inner = unsafe { self . map_unchecked_mut (| x | x . inner . get_mut ()) } ; inner . poll_next (cx) } }
};
}
