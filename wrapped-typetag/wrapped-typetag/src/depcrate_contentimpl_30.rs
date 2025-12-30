// Generated macro for impl_30 (impl)
macro_rules! Depcrate_contentimpl_30 {
() => {
// Module: crate::content
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'de > Content < 'de > { fn unexpected (& self) -> Unexpected { match * self { Content :: Bool (b) => Unexpected :: Bool (b) , Content :: U8 (n) => Unexpected :: Unsigned (u64 :: from (n)) , Content :: U16 (n) => Unexpected :: Unsigned (u64 :: from (n)) , Content :: U32 (n) => Unexpected :: Unsigned (u64 :: from (n)) , Content :: U64 (n) => Unexpected :: Unsigned (n) , Content :: I8 (n) => Unexpected :: Signed (i64 :: from (n)) , Content :: I16 (n) => Unexpected :: Signed (i64 :: from (n)) , Content :: I32 (n) => Unexpected :: Signed (i64 :: from (n)) , Content :: I64 (n) => Unexpected :: Signed (n) , Content :: F32 (f) => Unexpected :: Float (f64 :: from (f)) , Content :: F64 (f) => Unexpected :: Float (f) , Content :: Char (c) => Unexpected :: Char (c) , Content :: String (ref s) => Unexpected :: Str (s) , Content :: Str (s) => Unexpected :: Str (s) , Content :: ByteBuf (ref b) => Unexpected :: Bytes (b) , Content :: Bytes (b) => Unexpected :: Bytes (b) , Content :: None | Content :: Some (_) => Unexpected :: Option , Content :: Unit => Unexpected :: Unit , Content :: Newtype (_) => Unexpected :: NewtypeStruct , Content :: Seq (_) => Unexpected :: Seq , Content :: Map (_) => Unexpected :: Map , } } }
};
}
