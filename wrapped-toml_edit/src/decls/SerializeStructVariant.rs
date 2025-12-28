macro_rules! deps {
    () => {
        SerializeInlineTable!();
    };
}

macro_rules! SerializeStructVariant {
    () => {
        deps!();
        pub struct SerializeStructVariant { variant : & 'static str , inner : SerializeInlineTable , }
    };
}

SerializeStructVariant!()