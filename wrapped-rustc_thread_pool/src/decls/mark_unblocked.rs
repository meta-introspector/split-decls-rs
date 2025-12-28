macro_rules! deps {
    () => {
        Registry!();
    };
}

macro_rules! mark_unblocked {
    () => {
        deps!();
        # [doc = " Mark a previously blocked Rayon worker thread as unblocked"] # [inline] pub fn mark_unblocked (registry : & Registry) { registry . sleep . mark_unblocked () }
    };
}

mark_unblocked!()