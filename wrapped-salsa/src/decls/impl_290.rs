macro_rules! deps {
    () => {
        Storage!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < Db > Drop for Storage < Db > { fn drop (& mut self) { self . zalsa_local . record_unfilled_pages (self . handle . zalsa_impl . table ()) ; } }
    };
}

impl_290!()