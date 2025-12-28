macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for IntoIter < T > { fn next_back (& mut self) -> Option < T > { if self . start == self . vec . len () { None } else { self . vec . pop () } } }
    };
}

impl_63!()