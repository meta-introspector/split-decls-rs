macro_rules! deps {
    () => {
        Value!();
        FieldSet!();
        Field!();
    };
}

macro_rules! ValueSet {
    () => {
        deps!();
        # [doc = " A set of fields and values for a span."] pub struct ValueSet < 'a > { values : & 'a [(& 'a Field , Option < & 'a (dyn Value + 'a) >)] , fields : & 'a FieldSet , }
    };
}

ValueSet!();