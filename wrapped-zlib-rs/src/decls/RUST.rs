macro_rules! deps {
    () => {
        Allocator!();
    };
}

macro_rules! RUST {
    () => {
        deps!();
        # [cfg (feature = "rust-allocator")] pub static RUST : Allocator < 'static > = Allocator { zalloc : zalloc_rust , zfree : zfree_rust , opaque : core :: ptr :: null_mut () , _marker : PhantomData , } ;
    };
}

RUST!()