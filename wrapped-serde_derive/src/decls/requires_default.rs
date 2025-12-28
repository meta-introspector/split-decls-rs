macro_rules! deps {
    () => {
        Field!();
        Variant!();
        Default!();
    };
}

macro_rules! requires_default {
    () => {
        deps!();
        fn requires_default (field : & attr :: Field , _variant : Option < & attr :: Variant >) -> bool { if let attr :: Default :: Default = * field . default () { true } else { false } }
    };
}

requires_default!();