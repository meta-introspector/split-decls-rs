macro_rules! deps {
    () => {
        DefaultKind!();
    };
}

macro_rules! Details {
    () => {
        deps!();
        struct Details { signal : c_int , name : & 'static str , default_kind : DefaultKind , }
    };
}

Details!()