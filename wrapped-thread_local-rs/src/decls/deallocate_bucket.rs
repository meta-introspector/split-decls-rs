macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! deallocate_bucket {
    () => {
        deps!();
        # [doc = " # Safety"] # [doc = " The caller must ensure that `bucket` was allocated from [allocate_bucket]"] # [doc = " with the same `size` parameter."] unsafe fn deallocate_bucket < T > (bucket : * mut Entry < T > , size : usize) { let slice = unsafe { std :: slice :: from_raw_parts_mut (bucket , size) } ; drop (unsafe { Box :: from_raw (slice) }) ; }
    };
}

deallocate_bucket!();