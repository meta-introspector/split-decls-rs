// Generated macro for impl_394 (impl)
macro_rules! Depcrate_util_linked_listimpl_394 {
() => {
// Module: crate::util::linked_list
// Provides: {"impl_394"}
// Dependencies: {}
impl < T > Pointers < T > { # [doc = " Create a new set of empty pointers"] pub (crate) fn new () -> Pointers < T > { Pointers { inner : UnsafeCell :: new (PointersInner { prev : None , next : None , _pin : PhantomPinned , }) , } } pub (crate) fn get_prev (& self) -> Option < NonNull < T > > { unsafe { ptr :: addr_of ! ((* self . inner . get ()) . prev) . read () } } pub (crate) fn get_next (& self) -> Option < NonNull < T > > { unsafe { ptr :: addr_of ! ((* self . inner . get ()) . next) . read () } } fn set_prev (& mut self , value : Option < NonNull < T > >) { unsafe { ptr :: addr_of_mut ! ((* self . inner . get ()) . prev) . write (value) ; } } fn set_next (& mut self , value : Option < NonNull < T > >) { unsafe { ptr :: addr_of_mut ! ((* self . inner . get ()) . next) . write (value) ; } } }
};
}
