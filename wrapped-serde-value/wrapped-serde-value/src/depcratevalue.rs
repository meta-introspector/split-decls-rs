// Generated macro for Value (enum)
macro_rules! DepcrateValue {
() => {
// Module: crate
// Provides: {"Value"}
// Dependencies: {}
# [derive (Clone , Debug)] pub enum Value { Bool (bool) , U8 (u8) , U16 (u16) , U32 (u32) , U64 (u64) , I8 (i8) , I16 (i16) , I32 (i32) , I64 (i64) , F32 (f32) , F64 (f64) , Char (char) , String (String) , Unit , Option (Option < Box < Value > >) , Newtype (Box < Value >) , Seq (Vec < Value >) , Map (BTreeMap < Value , Value >) , Bytes (Vec < u8 >) , }
};
}
