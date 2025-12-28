macro_rules! deps {
    () => {
        TargetTuple!();
    };
}

macro_rules! impl_547 {
    () => {
        deps!();
        impl Hash for TargetTuple { fn hash < H : Hasher > (& self , state : & mut H) -> () { match self { TargetTuple :: TargetTuple (tuple) => { 0u8 . hash (state) ; tuple . hash (state) } TargetTuple :: TargetJson { path_for_rustdoc : _ , tuple , contents } => { 1u8 . hash (state) ; tuple . hash (state) ; contents . hash (state) } } } }
    };
}

impl_547!();