macro_rules! deps {
    () => {
        RawIndex!();
    };
}

macro_rules! IndexIter {
    () => {
        deps!();
        pub (crate) struct IndexIter < const N : usize > { pub (crate) remaining : usize , pub (crate) left_index : RawIndex < N > , pub (crate) right_index : RawIndex < N > , }
    };
}

IndexIter!();