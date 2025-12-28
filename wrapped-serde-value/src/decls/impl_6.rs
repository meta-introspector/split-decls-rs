macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Value { fn discriminant (& self) -> usize { match * self { Value :: Bool (..) => 0 , Value :: U8 (..) => 1 , Value :: U16 (..) => 2 , Value :: U32 (..) => 3 , Value :: U64 (..) => 4 , Value :: I8 (..) => 5 , Value :: I16 (..) => 6 , Value :: I32 (..) => 7 , Value :: I64 (..) => 8 , Value :: F32 (..) => 9 , Value :: F64 (..) => 10 , Value :: Char (..) => 11 , Value :: String (..) => 12 , Value :: Unit => 13 , Value :: Option (..) => 14 , Value :: Newtype (..) => 15 , Value :: Seq (..) => 16 , Value :: Map (..) => 17 , Value :: Bytes (..) => 18 , } } fn unexpected (& self) -> serde :: de :: Unexpected < '_ > { match * self { Value :: Bool (b) => serde :: de :: Unexpected :: Bool (b) , Value :: U8 (n) => serde :: de :: Unexpected :: Unsigned (n as u64) , Value :: U16 (n) => serde :: de :: Unexpected :: Unsigned (n as u64) , Value :: U32 (n) => serde :: de :: Unexpected :: Unsigned (n as u64) , Value :: U64 (n) => serde :: de :: Unexpected :: Unsigned (n) , Value :: I8 (n) => serde :: de :: Unexpected :: Signed (n as i64) , Value :: I16 (n) => serde :: de :: Unexpected :: Signed (n as i64) , Value :: I32 (n) => serde :: de :: Unexpected :: Signed (n as i64) , Value :: I64 (n) => serde :: de :: Unexpected :: Signed (n) , Value :: F32 (n) => serde :: de :: Unexpected :: Float (n as f64) , Value :: F64 (n) => serde :: de :: Unexpected :: Float (n) , Value :: Char (c) => serde :: de :: Unexpected :: Char (c) , Value :: String (ref s) => serde :: de :: Unexpected :: Str (s) , Value :: Unit => serde :: de :: Unexpected :: Unit , Value :: Option (_) => serde :: de :: Unexpected :: Option , Value :: Newtype (_) => serde :: de :: Unexpected :: NewtypeStruct , Value :: Seq (_) => serde :: de :: Unexpected :: Seq , Value :: Map (_) => serde :: de :: Unexpected :: Map , Value :: Bytes (ref b) => serde :: de :: Unexpected :: Bytes (b) , } } pub fn deserialize_into < 'de , T : Deserialize < 'de > > (self) -> Result < T , DeserializerError > { T :: deserialize (self) } }
    };
}

impl_6!()