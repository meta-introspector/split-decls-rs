macro_rules! MarkFrame {
    () => {
        pub struct MarkFrame < 'a > { index : SerializedDepNodeIndex , parent : Option < & 'a MarkFrame < 'a > > , }
    };
}

MarkFrame!()