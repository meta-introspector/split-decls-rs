macro_rules! FingerprintComponent {
    () => {
        pub trait FingerprintComponent { fn as_u64 (& self) -> u64 ; }
    };
}

FingerprintComponent!();