macro_rules! deps {
    () => {
        AncillaryIter!();
    };
}

macro_rules! impl_593 {
    () => {
        deps!();
        impl < 'data , T > Drop for AncillaryIter < 'data , T > { fn drop (& mut self) { self . for_each (drop) ; } }
    };
}

impl_593!();