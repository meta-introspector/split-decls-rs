// Generated macro for impl_819 (impl)
macro_rules! Depcrate_zerofrom_implsimpl_819 {
() => {
// Module: crate::zerofrom_impls
// Provides: {"impl_819"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'zf , 's , K0 , K1 , V > ZeroFrom < 'zf , ZeroMap2d < 's , K0 , K1 , V > > for ZeroMap2d < 'zf , K0 , K1 , V > where K0 : 'static + for < 'b > ZeroMapKV < 'b > + ? Sized , K1 : 'static + for < 'b > ZeroMapKV < 'b > + ? Sized , V : 'static + for < 'b > ZeroMapKV < 'b > + ? Sized , < K0 as ZeroMapKV < 'zf > > :: Container : ZeroFrom < 'zf , < K0 as ZeroMapKV < 's > > :: Container > , < K1 as ZeroMapKV < 'zf > > :: Container : ZeroFrom < 'zf , < K1 as ZeroMapKV < 's > > :: Container > , < V as ZeroMapKV < 'zf > > :: Container : ZeroFrom < 'zf , < V as ZeroMapKV < 's > > :: Container > , { fn zero_from (other : & 'zf ZeroMap2d < 's , K0 , K1 , V >) -> Self { ZeroMap2d { keys0 : K0 :: Container :: zero_from (& other . keys0) , joiner : ZeroVec :: zero_from (& other . joiner) , keys1 : K1 :: Container :: zero_from (& other . keys1) , values : V :: Container :: zero_from (& other . values) , } } }
};
}
