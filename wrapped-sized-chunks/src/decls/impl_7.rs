macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'a , A , T > DoubleEndedIterator for Drain < 'a , A , T > { fn next_back (& mut self) -> Option < Self :: Item > { self . array . pop () } }
    };
}

impl_7!();