macro_rules! deps {
    () => {
        Callsites!();
        DefaultCallsite!();
        Callsite!();
        LevelFilter!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl Callsites { # [doc = " Rebuild `Interest`s for all callsites in the registry."] # [doc = ""] # [doc = " This also re-computes the max level hint."] fn rebuild_interest (& self , dispatchers : dispatchers :: Rebuilder < '_ >) { let mut max_level = LevelFilter :: OFF ; dispatchers . for_each (| dispatch | { let level_hint = dispatch . max_level_hint () . unwrap_or (LevelFilter :: TRACE) ; if level_hint > max_level { max_level = level_hint ; } }) ; self . for_each (| callsite | { rebuild_callsite_interest (callsite , & dispatchers) ; }) ; LevelFilter :: set_max (max_level) ; } # [doc = " Push a `dyn Callsite` trait object to the callsite registry."] # [doc = ""] # [doc = " This will attempt to lock the callsites vector."] fn push_dyn (& self , callsite : & 'static dyn Callsite) { let mut lock = LOCKED_CALLSITES . lock () . unwrap () ; self . has_locked_callsites . store (true , Ordering :: Release) ; lock . push (callsite) ; } # [doc = " Push a `DefaultCallsite` to the callsite registry."] # [doc = ""] # [doc = " If we know the callsite being pushed is a `DefaultCallsite`, we can push"] # [doc = " it to the linked list without having to acquire a lock."] fn push_default (& self , callsite : & 'static DefaultCallsite) { let mut head = self . list_head . load (Ordering :: Acquire) ; loop { callsite . next . store (head , Ordering :: Release) ; assert_ne ! (callsite as * const _ , head , "Attempted to register a `DefaultCallsite` that already exists! \
                This will cause an infinite loop when attempting to read from the \
                callsite cache. This is likely a bug! You should only need to call \
                `DefaultCallsite::register` once per `DefaultCallsite`.") ; match self . list_head . compare_exchange (head , callsite as * const _ as * mut _ , Ordering :: AcqRel , Ordering :: Acquire ,) { Ok (_) => { break ; } Err (current) => head = current , } } } # [doc = " Invokes the provided closure `f` with each callsite in the registry."] fn for_each (& self , mut f : impl FnMut (& 'static dyn Callsite)) { let mut head = self . list_head . load (Ordering :: Acquire) ; while let Some (cs) = unsafe { head . as_ref () } { f (cs) ; head = cs . next . load (Ordering :: Acquire) ; } if self . has_locked_callsites . load (Ordering :: Acquire) { let locked = LOCKED_CALLSITES . lock () . unwrap () ; for & cs in locked . iter () { f (cs) ; } } } }
    };
}

impl_48!();