macro_rules! deps {
    () => {
        Config!();
        Ptr!();
        Shard!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < T , C : cfg :: Config > Ptr < T , C > { # [inline] fn null () -> Self { Self (AtomicPtr :: new (ptr :: null_mut ())) } # [inline] fn load (& self , order : Ordering) -> Option < & Shard < T , C > > { let ptr = self . 0 . load (order) ; test_println ! ("---> loaded={:p} (order={:?})" , ptr , order) ; if ptr . is_null () { test_println ! ("---> null") ; return None ; } let track = unsafe { & * ptr } ; Some (track . get_ref ()) } # [inline] fn set (& self , new : * mut alloc :: Track < Shard < T , C > >) { self . 0 . compare_exchange (ptr :: null_mut () , new , AcqRel , Acquire) . expect ("a shard can only be inserted by the thread that owns it, this is a bug!") ; } }
    };
}

impl_158!()