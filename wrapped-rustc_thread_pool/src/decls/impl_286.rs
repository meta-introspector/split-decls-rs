macro_rules! deps {
    () => {
        AbortIfPanic!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl Drop for AbortIfPanic { fn drop (& mut self) { eprintln ! ("Rayon: detected unexpected panic; aborting") ; :: std :: process :: abort () ; } }
    };
}

impl_286!()