macro_rules! deps {
    () => {
        Allocator!();
    };
}

macro_rules! FAIL {
    () => {
        deps!();
        # [cfg (test)] static FAIL : Allocator < 'static > = Allocator { zalloc : zalloc_fail , zfree : zfree_fail , opaque : core :: ptr :: null_mut () , _marker : PhantomData , } ;
    };
}

FAIL!()