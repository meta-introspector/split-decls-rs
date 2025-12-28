macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl PartialEq for ConstValue { fn eq (& self , other : & ConstValue) -> bool { match (self , other) { (ConstValue :: Null , ConstValue :: Null) => true , (ConstValue :: Number (a) , ConstValue :: Number (b)) => a == b , (ConstValue :: Boolean (a) , ConstValue :: Boolean (b)) => a == b , (ConstValue :: String (a) , ConstValue :: String (b)) => a == b , (ConstValue :: Enum (a) , ConstValue :: String (b)) => a == b , (ConstValue :: String (a) , ConstValue :: Enum (b)) => a == b , (ConstValue :: Enum (a) , ConstValue :: Enum (b)) => a == b , (ConstValue :: Binary (a) , ConstValue :: Binary (b)) => a == b , (ConstValue :: List (a) , ConstValue :: List (b)) => { if a . len () != b . len () { return false ; } a . iter () . zip (b . iter ()) . all (| (a , b) | a == b) } (ConstValue :: Object (a) , ConstValue :: Object (b)) => { if a . len () != b . len () { return false ; } for (a_key , a_value) in a . iter () { if let Some (b_value) = b . get (a_key . as_str ()) { if b_value != a_value { return false ; } } else { return false ; } } true } _ => false , } } }
    };
}

impl_98!();