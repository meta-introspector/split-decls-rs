// Generated macro for impl_818 (impl)
macro_rules! Depcrate_zerofrom_implsimpl_818 {
() => {
// Module: crate::zerofrom_impls
// Provides: {"impl_818"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'zf , 's , K , V > ZeroFrom < 'zf , ZeroMap < 's , K , V > > for ZeroMap < 'zf , K , V > where K : 'static + for < 'b > ZeroMapKV < 'b > + ? Sized , V : 'static + for < 'b > ZeroMapKV < 'b > + ? Sized , < K as ZeroMapKV < 'zf > > :: Container : ZeroFrom < 'zf , < K as ZeroMapKV < 's > > :: Container > , < V as ZeroMapKV < 'zf > > :: Container : ZeroFrom < 'zf , < V as ZeroMapKV < 's > > :: Container > , { fn zero_from (other : & 'zf ZeroMap < 's , K , V >) -> Self { ZeroMap { keys : K :: Container :: zero_from (& other . keys) , values : V :: Container :: zero_from (& other . values) , } } }
};
}
