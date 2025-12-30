// Generated macro for impl_2833 (impl)
macro_rules! Depcrate_pathimpl_2833 {
() => {
// Module: crate::path
// Provides: {"impl_2833"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a > DoubleEndedIterator for Components < 'a > { fn next_back (& mut self) -> Option < Component < 'a > > { while ! self . finished () { match self . back { State :: Body if self . path . len () > self . len_before_body () => { let (size , comp) = self . parse_next_component_back () ; self . path = & self . path [.. self . path . len () - size] ; if comp . is_some () { return comp ; } } State :: Body => { self . back = State :: StartDir ; } State :: StartDir => { self . back = State :: Prefix ; if self . has_physical_root { self . path = & self . path [.. self . path . len () - 1] ; return Some (Component :: RootDir) ; } else if let Some (p) = self . prefix { if p . has_implicit_root () && ! p . is_verbatim () { return Some (Component :: RootDir) ; } } else if self . include_cur_dir () { self . path = & self . path [.. self . path . len () - 1] ; return Some (Component :: CurDir) ; } } State :: Prefix if self . prefix_len () > 0 => { self . back = State :: Done ; return Some (Component :: Prefix (PrefixComponent { raw : unsafe { OsStr :: from_encoded_bytes_unchecked (self . path) } , parsed : self . prefix . unwrap () , })) ; } State :: Prefix => { self . back = State :: Done ; return None ; } State :: Done => unreachable ! () , } } None } }
};
}
