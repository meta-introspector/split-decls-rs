macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < 'a , T > DoubleEndedIterator for Drain < 'a , T > { fn next_back (& mut self) -> Option < T > { self . iter . next_back () . map (| x | unsafe { ptr :: read (x) }) } }
    };
}

impl_73!()