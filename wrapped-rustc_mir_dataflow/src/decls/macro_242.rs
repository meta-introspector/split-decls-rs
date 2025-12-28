macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! macro_242 {
    () => {
        deps!();
        rustc_index :: newtype_index ! (# [doc = " This index uniquely identifies a tracked place and therefore a slot in [`State`]."] # [doc = ""] # [doc = " It is an implementation detail of this module."] struct ValueIndex { }) ;
    };
}

macro_242!();