macro_rules! deps {
    () => {
        ByteSlice!();
    };
}

macro_rules! CloneableByteSlice {
    () => {
        deps!();
        # [doc = " A [`ByteSlice`] which can be cloned without violating dereference stability."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If `B: CloneableByteSlice`, then the dereference stability properties"] # [doc = " required by [`ByteSlice`] (see that trait's safety documentation) do not"] # [doc = " only hold regarding two calls to `b.deref()` or `b.deref_mut()`, but also"] # [doc = " hold regarding `c.deref()` or `c.deref_mut()`, where `c` is produced by"] # [doc = " `b.clone()`, `b.clone().clone()`, etc."] pub unsafe trait CloneableByteSlice : ByteSlice + Clone { }
    };
}

CloneableByteSlice!()