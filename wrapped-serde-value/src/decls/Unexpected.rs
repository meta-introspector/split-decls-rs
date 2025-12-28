macro_rules! Unexpected {
    () => {
        # [derive (Debug)] pub enum Unexpected { Bool (bool) , Unsigned (u64) , Signed (i64) , Float (f64) , Char (char) , Str (String) , Bytes (Vec < u8 >) , Unit , Option , NewtypeStruct , Seq , Map , Enum , UnitVariant , NewtypeVariant , TupleVariant , StructVariant , Other (String) , }
    };
}

Unexpected!();