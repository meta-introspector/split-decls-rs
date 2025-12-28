macro_rules! SerializeEvent {
    () => {
        # [doc = " Implements `serde::Serialize` to write `Event` data to a serializer."] # [derive (Debug)] pub struct SerializeEvent < 'a > (& 'a Event < 'a >) ;
    };
}

SerializeEvent!()