// Generated macro for impl_221 (impl)
macro_rules! Depcrate_itemimpl_221 {
() => {
// Module: crate::item
// Provides: {"impl_221"}
// Dependencies: {}
impl fmt :: Debug for SearchResult { # [cold] fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Ref (reference) => fmt . debug_struct ("SearchResult::Ref") . field ("reference" , reference) . finish () , Self :: Data (buf) => fmt . debug_struct ("SearchResult::Data") . field ("data" , buf) . finish () , Self :: Dict (_) => { let mut debug = fmt . debug_struct ("SearchResult::Dict") ; for (k , v) in self . simplify_dict () . unwrap () { debug . field (& k , & v) ; } debug . finish () } , Self :: Other => write ! (fmt , "SearchResult::Other") , } } }
};
}
