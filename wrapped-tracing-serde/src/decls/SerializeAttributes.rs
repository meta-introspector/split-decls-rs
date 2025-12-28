macro_rules! SerializeAttributes {
    () => {
        # [doc = " Implements `serde::Serialize` to write `Attributes` data to a serializer."] # [derive (Debug)] pub struct SerializeAttributes < 'a > (& 'a Attributes < 'a >) ;
    };
}

SerializeAttributes!()