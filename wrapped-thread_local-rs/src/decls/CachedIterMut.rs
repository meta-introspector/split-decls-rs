macro_rules! deps {
    () => {
        IterMut!();
        CachedThreadLocal!();
    };
}

macro_rules! CachedIterMut {
    () => {
        deps!();
        # [doc = " Mutable iterator over the contents of a `CachedThreadLocal`."] # [deprecated (since = "1.1.0" , note = "Use `IterMut` instead")] pub struct CachedIterMut < 'a , T : Send + 'a > { inner : IterMut < 'a , T > , }
    };
}

CachedIterMut!();