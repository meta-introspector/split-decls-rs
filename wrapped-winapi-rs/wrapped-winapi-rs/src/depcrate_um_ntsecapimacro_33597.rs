// Generated macro for macro_33597 (macro)
macro_rules! Depcrate_um_ntsecapimacro_33597 {
() => {
// Module: crate::um::ntsecapi
// Provides: {"macro_33597"}
// Dependencies: {}
STRUCT ! { struct KERB_SETPASSWORD_EX_REQUEST { MessageType : KERB_PROTOCOL_MESSAGE_TYPE , LogonId : LUID , CredentialsHandle : SecHandle , Flags : ULONG , AccountRealm : UNICODE_STRING , AccountName : UNICODE_STRING , Password : UNICODE_STRING , ClientRealm : UNICODE_STRING , ClientName : UNICODE_STRING , Impersonating : BOOLEAN , KdcAddress : UNICODE_STRING , KdcAddressType : ULONG , } }
};
}
