macro_rules! deps {
    () => {
        AncillaryIter!();
    };
}

macro_rules! impl_597 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for AncillaryIter < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { if self . data . len () < size_of :: < T > () { return None ; } let item = unsafe { let ptr = self . data . as_ptr () . add (self . data . len () - size_of :: < T > ()) ; ptr . cast :: < T > () . read_unaligned () } ; let len = self . data . len () ; let data = take (& mut self . data) ; self . data = & mut data [.. len - size_of :: < T > ()] ; Some (item) } }
    };
}

impl_597!()