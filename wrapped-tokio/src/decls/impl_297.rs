macro_rules! deps {
    () => {
        LinkedList!();
        Link!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        unsafe impl < L : Link > Sync for LinkedList < L , L :: Target > where L :: Target : Sync { }
    };
}

impl_297!()