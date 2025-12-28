macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < CTX > ToStableHashKey < CTX > for Symbol { type KeyType = String ; # [inline] fn to_stable_hash_key (& self , _ : & CTX) -> String { self . as_str () . to_string () } }
    };
}

impl_201!();