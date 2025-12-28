macro_rules! deps {
    () => {
        AtomicCell!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < T > AtomicCell < T > { pub (crate) fn new (data : Option < Box < T > >) -> AtomicCell < T > { AtomicCell { data : AtomicPtr :: new (to_raw (data)) , } } pub (crate) fn swap (& self , val : Option < Box < T > >) -> Option < Box < T > > { let old = self . data . swap (to_raw (val) , AcqRel) ; from_raw (old) } pub (crate) fn set (& self , val : Box < T >) { let _ = self . swap (Some (val)) ; } pub (crate) fn take (& self) -> Option < Box < T > > { self . swap (None) } }
    };
}

impl_276!()