macro_rules! SerializeMetadata {
    () => {
        # [derive (Debug)] pub struct SerializeMetadata < 'a > (& 'a Metadata < 'a >) ;
    };
}

SerializeMetadata!()