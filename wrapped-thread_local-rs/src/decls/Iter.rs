macro_rules! deps {
    () => {
        ThreadLocal!();
        RawIter!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " Iterator over the contents of a `ThreadLocal`."] # [derive (Debug)] pub struct Iter < 'a , T : Send + Sync > { thread_local : & 'a ThreadLocal < T > , raw : RawIter , }
    };
}

Iter!();