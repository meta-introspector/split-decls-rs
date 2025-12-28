macro_rules! deps {
    () => {
        FieldSet!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " An iterator over a set of fields."] # [derive (Debug)] pub struct Iter { idxs : Range < usize > , fields : FieldSet , }
    };
}

Iter!();