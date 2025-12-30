// Generated macro for InboundUnborrowedMessage (struct)
macro_rules! Depcrate_connInboundUnborrowedMessage {
() => {
// Module: crate::conn
// Provides: {"InboundUnborrowedMessage"}
// Dependencies: {}
# [doc = " An InboundPlainMessage which does not borrow its payload, but"] # [doc = " references a range that can later be borrowed."] struct InboundUnborrowedMessage { typ : ContentType , version : ProtocolVersion , bounds : Range < usize > , }
};
}
