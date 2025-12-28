macro_rules! ScopePtr {
    () => {
        # [doc = " Used to capture a scope `&Self` pointer in jobs, without faking a lifetime."] # [doc = ""] # [doc = " Unsafe code is still required to dereference the pointer, but that's fine in"] # [doc = " scope jobs that are guaranteed to execute before the scope ends."] struct ScopePtr < T > (* const T) ;
    };
}

ScopePtr!();