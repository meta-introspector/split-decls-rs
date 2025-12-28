macro_rules! deps {
    () => {
        StreamSafe!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > > StreamSafe < I > { # [doc = " Create a new stream safe iterator."] # [doc = ""] # [doc = " Note that this iterator can also be obtained by directly calling [`.stream_safe()`](crate::UnicodeNormalization::stream_safe)"] # [doc = " on the iterator."] # [inline] pub fn new (iter : I) -> Self { Self { iter , nonstarter_count : 0 , buffer : None , } } }
    };
}

impl_85!();