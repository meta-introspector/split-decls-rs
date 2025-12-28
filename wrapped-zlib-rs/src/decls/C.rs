macro_rules! deps {
    () => {
        Allocator!();
    };
}

macro_rules! C {
    () => {
        deps!();
        # [cfg (feature = "c-allocator")] pub static C : Allocator < 'static > = Allocator { zalloc : zalloc_c , zfree : zfree_c , opaque : core :: ptr :: null_mut () , _marker : PhantomData , } ;
    };
}

C!()