macro_rules! deps {
    () => {
        Unexpected!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Unexpected { pub fn to_unexpected < 'a > (& 'a self) -> de :: Unexpected < 'a > { match * self { Unexpected :: Bool (v) => de :: Unexpected :: Bool (v) , Unexpected :: Unsigned (v) => de :: Unexpected :: Unsigned (v) , Unexpected :: Signed (v) => de :: Unexpected :: Signed (v) , Unexpected :: Float (v) => de :: Unexpected :: Float (v) , Unexpected :: Char (v) => de :: Unexpected :: Char (v) , Unexpected :: Str (ref v) => de :: Unexpected :: Str (v) , Unexpected :: Bytes (ref v) => de :: Unexpected :: Bytes (v) , Unexpected :: Unit => de :: Unexpected :: Unit , Unexpected :: Option => de :: Unexpected :: Option , Unexpected :: NewtypeStruct => de :: Unexpected :: NewtypeStruct , Unexpected :: Seq => de :: Unexpected :: Seq , Unexpected :: Map => de :: Unexpected :: Map , Unexpected :: Enum => de :: Unexpected :: Enum , Unexpected :: UnitVariant => de :: Unexpected :: UnitVariant , Unexpected :: NewtypeVariant => de :: Unexpected :: NewtypeVariant , Unexpected :: TupleVariant => de :: Unexpected :: TupleVariant , Unexpected :: StructVariant => de :: Unexpected :: StructVariant , Unexpected :: Other (ref v) => de :: Unexpected :: Other (v) , } } }
    };
}

impl_2!()