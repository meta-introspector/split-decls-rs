macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! derive {
    () => {
        deps!();
        pub mod derive { pub use typesize_derive :: TypeSize ; }
    };
}

derive!();