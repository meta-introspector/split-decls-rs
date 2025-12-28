macro_rules! SerializeRecord {
    () => {
        # [doc = " Implements `serde::Serialize` to write `Record` data to a serializer."] # [derive (Debug)] pub struct SerializeRecord < 'a > (& 'a Record < 'a >) ;
    };
}

SerializeRecord!()