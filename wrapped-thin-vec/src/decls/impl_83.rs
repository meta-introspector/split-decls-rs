macro_rules! deps {
    () => {
        Splice!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < I : Iterator > DoubleEndedIterator for Splice < '_ , I > { fn next_back (& mut self) -> Option < Self :: Item > { self . drain . next_back () } }
    };
}

impl_83!();