macro_rules! deps {
    () => {
        ValueSet!();
    };
}

macro_rules! Record {
    () => {
        deps!();
        # [doc = " A set of fields recorded by a span."] # [derive (Debug)] pub struct Record < 'a > { values : & 'a field :: ValueSet < 'a > , }
    };
}

Record!();