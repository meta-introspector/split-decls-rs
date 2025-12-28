macro_rules! StringSubscriber {
    () => {
        pub (crate) struct StringSubscriber (& 'static str) ;
    };
}

StringSubscriber!();