macro_rules! deps {
    () => {
        StreamStatus!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl StreamStatus { fn is_ok (& self) -> bool { match self { Self :: Ok => true , Self :: Failure (_) | Self :: Expected (_) => false , } } }
    };
}

impl_67!();