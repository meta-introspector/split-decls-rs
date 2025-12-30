// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for TryLock < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { struct LockedPlaceholder ; impl fmt :: Debug for LockedPlaceholder { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("<locked>") } } let mut builder = f . debug_struct ("TryLock") ; if let Some (locked) = self . try_lock () { builder . field ("value" , & * locked) ; } else { builder . field ("value" , & LockedPlaceholder) ; } builder . finish () } }
};
}
