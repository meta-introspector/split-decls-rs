macro_rules! Value {
    () => {
        # [repr (C)] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) struct Value { a : u16 , b : u16 , }
    };
}

Value!()