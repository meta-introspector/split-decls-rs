macro_rules! deps {
    () => {
        Result!();
        Stream!();
        Value!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; # [test] fn stream_computed () { struct ComputedValue (usize) ; impl Value for ComputedValue { fn stream < 'sval , S : Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> Result { if self . 0 == 0 { stream . bool (true) } else { stream . value_computed (& ComputedValue (self . 0 - 1)) } } } assert_eq ! (true , ComputedValue (5) . to_bool () . unwrap ()) ; } }
    };
}

tests!();