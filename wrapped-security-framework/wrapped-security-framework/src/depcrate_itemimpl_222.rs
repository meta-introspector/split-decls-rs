// Generated macro for impl_222 (impl)
macro_rules! Depcrate_itemimpl_222 {
() => {
// Module: crate::item
// Provides: {"impl_222"}
// Dependencies: {}
impl SearchResult { # [doc = " If the search result is a `CFDict`, simplify that to a"] # [doc = " `HashMap<String, String>`. This transformation isn't"] # [doc = " comprehensive, it only supports `CFString`, `CFDate`, and `CFData`"] # [doc = " value types."] # [must_use] pub fn simplify_dict (& self) -> Option < HashMap < String , String > > { match self { Self :: Dict (d) => unsafe { let mut retmap = HashMap :: new () ; let (keys , values) = d . get_keys_and_values () ; for (k , v) in keys . iter () . zip (values . iter ()) { let keycfstr = CFString :: wrap_under_get_rule ((* k) . cast ()) ; let val : String = match CFGetTypeID (* v) { cfstring if cfstring == CFString :: type_id () => { format ! ("{}" , CFString :: wrap_under_get_rule ((* v) . cast ())) } , cfdata if cfdata == CFData :: type_id () => { let buf = CFData :: wrap_under_get_rule ((* v) . cast ()) ; let mut vec = Vec :: new () ; vec . extend_from_slice (buf . bytes ()) ; format ! ("{}" , String :: from_utf8_lossy (& vec)) } , cfdate if cfdate == CFDate :: type_id () => { format ! ("{}" , CFString :: wrap_under_create_rule (CFCopyDescription (* v))) } , _ => String :: from ("unknown") , } ; retmap . insert (format ! ("{keycfstr}") , val) ; } Some (retmap) } , _ => None , } } }
};
}
