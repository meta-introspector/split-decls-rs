// Generated macro for encode_region (function)
macro_rules! Depcrate_cfi_typeid_itanium_cxx_abi_encodeencode_region {
() => {
// Module: crate::cfi::typeid::itanium_cxx_abi::encode
// Provides: {"encode_region"}
// Dependencies: {}
# [doc = " Encodes a region using the Itanium C++ ABI as a vendor extended type."] fn encode_region < 'tcx > (region : Region < 'tcx > , dict : & mut FxHashMap < DictKey < 'tcx > , usize >) -> String { let mut s = String :: new () ; match region . kind () { RegionKind :: ReBound (debruijn , r) => { s . push_str ("u6regionI") ; let num = debruijn . index () as u64 ; if num > 0 { s . push_str (& to_disambiguator (num)) ; } let _ = write ! (s , "{}" , r . var . index () as u64) ; s . push ('E') ; compress (dict , DictKey :: Region (region) , & mut s) ; } RegionKind :: ReErased => { s . push_str ("u6region") ; compress (dict , DictKey :: Region (region) , & mut s) ; } RegionKind :: ReEarlyParam (..) | RegionKind :: ReLateParam (..) | RegionKind :: ReStatic | RegionKind :: ReError (_) | RegionKind :: ReVar (..) | RegionKind :: RePlaceholder (..) => { bug ! ("encode_region: unexpected `{:?}`" , region . kind ()) ; } } s }
};
}
