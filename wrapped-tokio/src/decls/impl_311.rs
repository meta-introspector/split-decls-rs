macro_rules! deps {
    () => {
        PointersInner!();
        UnsafeCell!();
        Pointers!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl < T > Pointers < T > { # [doc = " Create a new set of empty pointers"] pub (crate) fn new () -> Pointers < T > { Pointers { inner : UnsafeCell :: new (PointersInner { prev : None , next : None , _pin : PhantomPinned , }) , } } pub (crate) fn get_prev (& self) -> Option < NonNull < T > > { unsafe { ptr :: addr_of ! ((* self . inner . get ()) . prev) . read () } } pub (crate) fn get_next (& self) -> Option < NonNull < T > > { unsafe { ptr :: addr_of ! ((* self . inner . get ()) . next) . read () } } fn set_prev (& mut self , value : Option < NonNull < T > >) { unsafe { ptr :: addr_of_mut ! ((* self . inner . get ()) . prev) . write (value) ; } } fn set_next (& mut self , value : Option < NonNull < T > >) { unsafe { ptr :: addr_of_mut ! ((* self . inner . get ()) . next) . write (value) ; } } }
    };
}

impl_311!()