macro_rules! deps {
    () => {
        RustcPatCtxt!();
    };
}

macro_rules! RustcPatCtxtState {
    () => {
        deps!();
        # [doc = " Private fields of [`RustcPatCtxt`], separated out to permit record initialization syntax."] # [derive (Clone , Default)] pub struct RustcPatCtxtState { # [doc = " Has a deref pattern been lowered? This is initialized to `false` and is updated by"] # [doc = " [`RustcPatCtxt::lower_pat`] in order to avoid performing deref-pattern-specific validation"] # [doc = " for everything containing patterns."] has_lowered_deref_pat : Cell < bool > , }
    };
}

RustcPatCtxtState!()