macro_rules! Arch {
    () => {
        pub (crate) enum Arch { Aarch64 , I586 , X86_64 , }
    };
}

Arch!()