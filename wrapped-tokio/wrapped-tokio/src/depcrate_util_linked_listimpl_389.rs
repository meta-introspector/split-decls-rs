// Generated macro for impl_389 (impl)
macro_rules! Depcrate_util_linked_listimpl_389 {
() => {
// Module: crate::util::linked_list
// Provides: {"impl_389"}
// Dependencies: {}
# [cfg (any (feature = "fs" , feature = "rt" , all (unix , feature = "process") , feature = "signal" , feature = "sync" ,))] impl < L : Link > LinkedList < L , L :: Target > { pub (crate) fn last (& self) -> Option < & L :: Target > { let tail = self . tail . as_ref () ? ; unsafe { Some (& * tail . as_ptr ()) } } }
};
}
