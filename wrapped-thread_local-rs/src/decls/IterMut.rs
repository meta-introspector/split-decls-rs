macro_rules! deps {
    () => {
        ThreadLocal!();
        RawIter!();
    };
}

macro_rules! IterMut {
    () => {
        deps!();
        # [doc = " Mutable iterator over the contents of a `ThreadLocal`."] pub struct IterMut < 'a , T : Send > { thread_local : & 'a mut ThreadLocal < T > , raw : RawIter , }
    };
}

IterMut!()