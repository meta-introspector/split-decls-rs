macro_rules! deps {
    () => {
        Cx!();
        ProvisionalCacheEntry!();
        AvailableDepth!();
        Stack!();
        Delegate!();
    };
}

macro_rules! SearchGraph {
    () => {
        deps!();
        pub struct SearchGraph < D : Delegate < Cx = X > , X : Cx = < D as Delegate > :: Cx > { root_depth : AvailableDepth , stack : Stack < X > , # [doc = " The provisional cache contains entries for already computed goals which"] # [doc = " still depend on goals higher-up in the stack. We don't move them to the"] # [doc = " global cache and track them locally instead. A provisional cache entry"] # [doc = " is only valid until the result of one of its cycle heads changes."] provisional_cache : HashMap < X :: Input , Vec < ProvisionalCacheEntry < X > > > , _marker : PhantomData < D > , }
    };
}

SearchGraph!();