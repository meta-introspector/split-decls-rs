macro_rules! WithMinOptLevel {
    () => {
        pub (super) struct WithMinOptLevel < T > (pub u32 , pub T) ;
    };
}

WithMinOptLevel!()