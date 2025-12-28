macro_rules! deps {
    () => {
        ValueSet!();
        Subscriber!();
        Parent!();
        Metadata!();
    };
}

macro_rules! Attributes {
    () => {
        deps!();
        # [doc = " Attributes provided to a `Subscriber` describing a new span when it is"] # [doc = " created."] # [derive (Debug)] pub struct Attributes < 'a > { metadata : & 'static Metadata < 'static > , values : & 'a field :: ValueSet < 'a > , parent : Parent , }
    };
}

Attributes!()