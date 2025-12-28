macro_rules! SpanState {
    () => {
        pub (crate) struct SpanState { id : Id , name : & 'static str , refs : usize , meta : & 'static Metadata < 'static > , }
    };
}

SpanState!()