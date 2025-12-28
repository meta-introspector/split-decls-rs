macro_rules! deps {
    () => {
        ExpectedValue!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl PartialEq for ExpectedValue { fn eq (& self , other : & Self) -> bool { use ExpectedValue :: * ; match (self , other) { (F64 (a) , F64 (b)) => { debug_assert ! (! a . is_nan ()) ; debug_assert ! (! b . is_nan ()) ; a . eq (b) } (I64 (a) , I64 (b)) => a . eq (b) , (U64 (a) , U64 (b)) => a . eq (b) , (Bool (a) , Bool (b)) => a . eq (b) , (Str (a) , Str (b)) => a . eq (b) , (Debug (a) , Debug (b)) => a . eq (b) , (Any , _) => true , (_ , Any) => true , _ => false , } } }
    };
}

impl_29!();