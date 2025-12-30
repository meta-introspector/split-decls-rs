// Generated macro for VisitorStream (struct)
macro_rules! Depcrate_internal_sval_v2VisitorStream {
() => {
// Module: crate::internal::sval::v2
// Provides: {"VisitorStream"}
// Dependencies: {}
struct VisitorStream < 'a , 'v > { visitor : & 'a mut dyn InternalVisitor < 'v > , text_buf : value_bag_sval2 :: buffer :: TextBuf < 'v > , }
};
}
