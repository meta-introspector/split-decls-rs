// Generated macro for impl_396 (impl)
macro_rules! Depcrate_stream_rangeimpl_396 {
() => {
// Module: crate::stream::range
// Provides: {"impl_396"}
// Dependencies: {}
impl core :: fmt :: Display for Range { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . start_inclusive . fmt (f) ? ; match self . end_inclusive { Some (e) if e == self . start_inclusive => { } Some (e) => { "..=" . fmt (f) ? ; e . fmt (f) ? ; } None => { ".." . fmt (f) ? ; } } Ok (()) } }
};
}
