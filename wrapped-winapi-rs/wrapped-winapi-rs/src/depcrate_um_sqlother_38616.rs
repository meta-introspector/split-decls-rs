// Generated macro for other_38616 (other)
macro_rules! Depcrate_um_sqlother_38616 {
() => {
// Module: crate::um::sql
// Provides: {"other_38616"}
// Dependencies: {}
extern "system" { pub fn SQLAllocHandle (handleType : SQLSMALLINT , inputHandle : SQLHANDLE , outputHandle : * mut SQLHANDLE ,) -> SQLRETURN ; pub fn SQLDisconnect (connectionHandle : SQLHDBC ,) -> SQLRETURN ; pub fn SQLFetch (statementHandle : SQLHSTMT ,) -> SQLRETURN ; pub fn SQLFreeHandle (handleType : SQLSMALLINT , handle : SQLHANDLE ,) -> SQLRETURN ; pub fn SQLFreeStmt (statementHandle : SQLHSTMT , option : SQLUSMALLINT ,) -> SQLRETURN ; pub fn SQLGetData (statementHandle : SQLHSTMT , columnNumber : SQLUSMALLINT , targetType : SQLSMALLINT , targetValue : SQLPOINTER , bufferLength : SQLLEN , strLen_or_IndPtr : * mut SQLLEN ,) -> SQLRETURN ; pub fn SQLNumResultCols (statementHandle : SQLHSTMT , columnCount : * mut SQLSMALLINT ,) -> SQLRETURN ; pub fn SQLRowCount (statementHandle : SQLHSTMT , rowCount : * mut SQLLEN ,) -> SQLRETURN ; pub fn SQLSetConnectAttr (connectionHandle : SQLHDBC , attribute : SQLINTEGER , value : SQLPOINTER , stringLength : SQLINTEGER ,) -> SQLRETURN ; pub fn SQLSetEnvAttr (environmentHandle : SQLHENV , attribute : SQLINTEGER , value : SQLPOINTER , stringLength : SQLINTEGER ,) -> SQLRETURN ; }
};
}
