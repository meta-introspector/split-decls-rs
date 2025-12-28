macro_rules! DropTest {
    () => {
        pub (crate) struct DropTest < 'a > { counter : & 'a AtomicUsize , }
    };
}

DropTest!();