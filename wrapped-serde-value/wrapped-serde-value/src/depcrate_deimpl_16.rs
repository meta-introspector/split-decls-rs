// Generated macro for impl_16 (impl)
macro_rules! Depcrate_deimpl_16 {
() => {
// Module: crate::de
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'a > From < de :: Unexpected < 'a > > for Unexpected { fn from (unexp : de :: Unexpected) -> Unexpected { match unexp { de :: Unexpected :: Bool (v) => Unexpected :: Bool (v) , de :: Unexpected :: Unsigned (v) => Unexpected :: Unsigned (v) , de :: Unexpected :: Signed (v) => Unexpected :: Signed (v) , de :: Unexpected :: Float (v) => Unexpected :: Float (v) , de :: Unexpected :: Char (v) => Unexpected :: Char (v) , de :: Unexpected :: Str (v) => Unexpected :: Str (v . to_owned ()) , de :: Unexpected :: Bytes (v) => Unexpected :: Bytes (v . to_owned ()) , de :: Unexpected :: Unit => Unexpected :: Unit , de :: Unexpected :: Option => Unexpected :: Option , de :: Unexpected :: NewtypeStruct => Unexpected :: NewtypeStruct , de :: Unexpected :: Seq => Unexpected :: Seq , de :: Unexpected :: Map => Unexpected :: Map , de :: Unexpected :: Enum => Unexpected :: Enum , de :: Unexpected :: UnitVariant => Unexpected :: UnitVariant , de :: Unexpected :: NewtypeVariant => Unexpected :: NewtypeVariant , de :: Unexpected :: TupleVariant => Unexpected :: TupleVariant , de :: Unexpected :: StructVariant => Unexpected :: StructVariant , de :: Unexpected :: Other (v) => Unexpected :: Other (v . to_owned ()) , } } }
};
}
