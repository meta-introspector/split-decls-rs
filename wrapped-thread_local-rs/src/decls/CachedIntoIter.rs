macro_rules! deps {
    () => {
        CachedThreadLocal!();
        IntoIter!();
    };
}

macro_rules! CachedIntoIter {
    () => {
        deps!();
        # [doc = " An iterator that moves out of a `CachedThreadLocal`."] # [deprecated (since = "1.1.0" , note = "Use `IntoIter` instead")] pub struct CachedIntoIter < T : Send > { inner : IntoIter < T > , }
    };
}

CachedIntoIter!()