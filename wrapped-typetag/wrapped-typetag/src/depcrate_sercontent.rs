// Generated macro for Content (enum)
macro_rules! Depcrate_serContent {
() => {
// Module: crate::ser
// Provides: {"Content"}
// Dependencies: {}
# [derive (Debug)] pub enum Content { Bool (bool) , U8 (u8) , U16 (u16) , U32 (u32) , U64 (u64) , U128 (u128) , I8 (i8) , I16 (i16) , I32 (i32) , I64 (i64) , I128 (i128) , F32 (f32) , F64 (f64) , Char (char) , String (String) , Bytes (Vec < u8 >) , None , Some (Box < Content >) , Unit , UnitStruct (& 'static str) , UnitVariant (& 'static str , u32 , & 'static str) , NewtypeStruct (& 'static str , Box < Content >) , NewtypeVariant (& 'static str , u32 , & 'static str , Box < Content >) , Seq (Vec < Content >) , Tuple (Vec < Content >) , TupleStruct (& 'static str , Vec < Content >) , TupleVariant (& 'static str , u32 , & 'static str , Vec < Content >) , Map (Vec < (Content , Content) >) , Struct (& 'static str , Vec < (& 'static str , Content) >) , StructVariant (& 'static str , u32 , & 'static str , Vec < (& 'static str , Content) > ,) , }
};
}
