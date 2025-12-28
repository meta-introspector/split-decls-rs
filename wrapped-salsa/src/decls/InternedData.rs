macro_rules! InternedData {
    () => {
        pub trait InternedData : Sized + Eq + Hash + Clone + Sync + Send { }
    };
}

InternedData!();