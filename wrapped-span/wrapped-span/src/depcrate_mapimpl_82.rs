// Generated macro for impl_82 (impl)
macro_rules! Depcrate_mapimpl_82 {
() => {
// Module: crate::map
// Provides: {"impl_82"}
// Dependencies: {}
impl fmt :: Display for RealSpanMap { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "RealSpanMap({:?}):" , self . file_id) ? ; for span in self . pairs . iter () { writeln ! (f , "{}: {:#?}" , u32 :: from (span . 0) , span . 1) ? ; } Ok (()) } }
};
}
