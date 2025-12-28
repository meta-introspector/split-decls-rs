macro_rules! deps {
    () => {
        ThreadLocal!();
    };
}

macro_rules! CachedThreadLocal {
    () => {
        deps!();
        # [doc = " Wrapper around [`ThreadLocal`]."] # [doc = ""] # [doc = " This used to add a fast path for a single thread, however that has been"] # [doc = " obsoleted by performance improvements to [`ThreadLocal`] itself."] # [deprecated (since = "1.1.0" , note = "Use `ThreadLocal` instead")] pub struct CachedThreadLocal < T : Send > { inner : ThreadLocal < T > , }
    };
}

CachedThreadLocal!();