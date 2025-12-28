macro_rules! deps {
    () => {
        LinkedList!();
        Link!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        # [cfg (any (feature = "fs" , feature = "rt" , all (unix , feature = "process") , feature = "signal" , feature = "sync" ,))] impl < L : Link > LinkedList < L , L :: Target > { pub (crate) fn last (& self) -> Option < & L :: Target > { let tail = self . tail . as_ref () ? ; unsafe { Some (& * tail . as_ptr ()) } } }
    };
}

impl_306!();