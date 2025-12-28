macro_rules! deps {
    () => {
        Header!();
    };
}

macro_rules! alloc_align {
    () => {
        deps!();
        # [doc = " Gets the align necessary to allocate a `ThinVec<T>`"] fn alloc_align < T > () -> usize { max (mem :: align_of :: < T > () , mem :: align_of :: < Header > ()) }
    };
}

alloc_align!()