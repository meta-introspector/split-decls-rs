macro_rules! deps {
    () => {
        Lifecycle!();
        Config!();
        LifecycleGen!();
        Slot!();
        RefCount!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < T , C : cfg :: Config > fmt :: Debug for Slot < T , C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let lifecycle = self . lifecycle . load (Ordering :: Relaxed) ; f . debug_struct ("Slot") . field ("lifecycle" , & format_args ! ("{:#x}" , lifecycle)) . field ("state" , & Lifecycle :: < C > :: from_packed (lifecycle) . state) . field ("gen" , & LifecycleGen :: < C > :: from_packed (lifecycle) . 0) . field ("refs" , & RefCount :: < C > :: from_packed (lifecycle)) . field ("next" , & self . next ()) . finish () } }
    };
}

impl_88!()