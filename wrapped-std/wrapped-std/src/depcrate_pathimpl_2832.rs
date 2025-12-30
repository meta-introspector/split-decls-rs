// Generated macro for impl_2832 (impl)
macro_rules! Depcrate_pathimpl_2832 {
() => {
// Module: crate::path
// Provides: {"impl_2832"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a > Iterator for Components < 'a > { type Item = Component < 'a > ; fn next (& mut self) -> Option < Component < 'a > > { while ! self . finished () { match self . front { State :: Prefix if self . prefix_len () > 0 => { self . front = State :: StartDir ; debug_assert ! (self . prefix_len () <= self . path . len ()) ; let raw = & self . path [.. self . prefix_len ()] ; self . path = & self . path [self . prefix_len () ..] ; return Some (Component :: Prefix (PrefixComponent { raw : unsafe { OsStr :: from_encoded_bytes_unchecked (raw) } , parsed : self . prefix . unwrap () , })) ; } State :: Prefix => { self . front = State :: StartDir ; } State :: StartDir => { self . front = State :: Body ; if self . has_physical_root { debug_assert ! (! self . path . is_empty ()) ; self . path = & self . path [1 ..] ; return Some (Component :: RootDir) ; } else if let Some (p) = self . prefix { if p . has_implicit_root () && ! p . is_verbatim () { return Some (Component :: RootDir) ; } } else if self . include_cur_dir () { debug_assert ! (! self . path . is_empty ()) ; self . path = & self . path [1 ..] ; return Some (Component :: CurDir) ; } } State :: Body if ! self . path . is_empty () => { let (size , comp) = self . parse_next_component () ; self . path = & self . path [size ..] ; if comp . is_some () { return comp ; } } State :: Body => { self . front = State :: Done ; } State :: Done => unreachable ! () , } } None } }
};
}
