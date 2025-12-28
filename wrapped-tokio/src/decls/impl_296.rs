macro_rules! deps {
    () => {
        Link!();
        LinkedList!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        unsafe impl < L : Link > Send for LinkedList < L , L :: Target > where L :: Target : Send { }
    };
}

impl_296!();