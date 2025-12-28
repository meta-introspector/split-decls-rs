macro_rules! deps {
    () => {
        Callsite!();
        DefaultCallsite!();
    };
}

macro_rules! register {
    () => {
        deps!();
        # [doc = " Register a new [`Callsite`] with the global registry."] # [doc = ""] # [doc = " This should be called once per callsite after the callsite has been"] # [doc = " constructed."] # [doc = ""] # [doc = " See the [documentation on callsite registration][reg-docs] for details"] # [doc = " on the global callsite registry."] # [doc = ""] # [doc = " [`Callsite`]: crate::callsite::Callsite"] # [doc = " [reg-docs]: crate::callsite#registering-callsites"] pub fn register (callsite : & 'static dyn Callsite) { if callsite . private_type_id (private :: Private (())) . 0 == TypeId :: of :: < DefaultCallsite > () { let callsite = unsafe { & * (callsite as * const dyn Callsite as * const DefaultCallsite) } ; CALLSITES . push_default (callsite) ; } else { CALLSITES . push_dyn (callsite) ; } rebuild_callsite_interest (callsite , & DISPATCHERS . rebuilder ()) ; }
    };
}

register!();