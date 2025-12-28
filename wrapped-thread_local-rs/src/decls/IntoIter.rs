macro_rules! deps {
    () => {
        ThreadLocal!();
        RawIter!();
    };
}

macro_rules! IntoIter {
    () => {
        deps!();
        # [doc = " An iterator that moves out of a `ThreadLocal`."] # [derive (Debug)] pub struct IntoIter < T : Send > { thread_local : ThreadLocal < T > , raw : RawIter , }
    };
}

IntoIter!()