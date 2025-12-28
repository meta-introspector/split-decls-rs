macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Span { fn new (range : Range < usize > , data : & [u8]) -> Self { Self { range , data : data . into () , committed : false , } } # [doc = " Returns `true` if and only if this is a \"pure\" insertion,"] # [doc = " i.e. does not remove any existing data."] # [doc = ""] # [doc = " The insertion point is the `start` position of the range."] fn is_insert (& self) -> bool { self . range . start == self . range . end } }
    };
}

impl_11!()