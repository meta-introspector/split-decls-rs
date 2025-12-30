// Generated macro for to_idxs (function)
macro_rules! Depcrate_arbitraryto_idxs {
() => {
// Module: crate::arbitrary
// Provides: {"to_idxs"}
// Dependencies: {}
fn to_idxs (vals : & BTreeMap < FieldKey , Span > , key_to_idx : & HashMap < FieldKey , usize > ,) -> Result < Vec < usize > > { let mut idxs = Vec :: new () ; for (key , & span) in vals { if let Some (& idx) = key_to_idx . get (key) { idxs . push (idx) ; } else { bail ! (span , "cannot find value `#{}` in this scope." , key) ; } } idxs . sort_unstable () ; Ok (idxs) }
};
}
