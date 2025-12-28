macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl Ord for Key { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . get () . cmp (other . get ()) } }
    };
}

impl_132!();