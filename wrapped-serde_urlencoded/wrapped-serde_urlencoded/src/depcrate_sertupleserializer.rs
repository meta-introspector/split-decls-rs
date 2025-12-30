// Generated macro for TupleSerializer (struct)
macro_rules! Depcrate_serTupleSerializer {
() => {
// Module: crate::ser
// Provides: {"TupleSerializer"}
// Dependencies: {}
# [doc = " Tuple serializer."] # [doc = ""] # [doc = " Mostly used for arrays."] pub struct TupleSerializer < 'input , 'output , Target : UrlEncodedTarget > { urlencoder : & 'output mut UrlEncodedSerializer < 'input , Target > , }
};
}
