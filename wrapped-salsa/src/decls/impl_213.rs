macro_rules! deps {
    () => {
        HashEqLike!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        # [cfg (feature = "compact_str")] impl HashEqLike < & str > for compact_str :: CompactString { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (self , & mut * h) } fn eq (& self , data : & & str) -> bool { self == * data } }
    };
}

impl_213!()