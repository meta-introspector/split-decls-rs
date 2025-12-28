macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Hash for Value { fn hash < H > (& self , hasher : & mut H) where H : Hasher , { self . discriminant () . hash (hasher) ; match * self { Value :: Bool (v) => v . hash (hasher) , Value :: U8 (v) => v . hash (hasher) , Value :: U16 (v) => v . hash (hasher) , Value :: U32 (v) => v . hash (hasher) , Value :: U64 (v) => v . hash (hasher) , Value :: I8 (v) => v . hash (hasher) , Value :: I16 (v) => v . hash (hasher) , Value :: I32 (v) => v . hash (hasher) , Value :: I64 (v) => v . hash (hasher) , Value :: F32 (v) => OrderedFloat (v) . hash (hasher) , Value :: F64 (v) => OrderedFloat (v) . hash (hasher) , Value :: Char (v) => v . hash (hasher) , Value :: String (ref v) => v . hash (hasher) , Value :: Unit => () . hash (hasher) , Value :: Option (ref v) => v . hash (hasher) , Value :: Newtype (ref v) => v . hash (hasher) , Value :: Seq (ref v) => v . hash (hasher) , Value :: Map (ref v) => v . hash (hasher) , Value :: Bytes (ref v) => v . hash (hasher) , } } }
    };
}

impl_47!()