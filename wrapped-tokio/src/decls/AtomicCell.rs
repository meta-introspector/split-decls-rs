macro_rules! AtomicCell {
    () => {
        pub (crate) struct AtomicCell < T > { data : AtomicPtr < T > , }
    };
}

AtomicCell!()