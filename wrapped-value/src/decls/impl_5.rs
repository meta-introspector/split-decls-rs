macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl ConstValue { # [inline] fn unexpected (& self) -> Unexpected { match self { ConstValue :: Null => Unexpected :: Unit , ConstValue :: Number (_) => Unexpected :: Other ("number") , ConstValue :: String (v) => Unexpected :: Str (v) , ConstValue :: Boolean (v) => Unexpected :: Bool (* v) , ConstValue :: Binary (v) => Unexpected :: Bytes (v) , ConstValue :: Enum (v) => Unexpected :: Str (v) , ConstValue :: List (_) => Unexpected :: Seq , ConstValue :: Object (_) => Unexpected :: Map , } } }
    };
}

impl_5!()