// Generated macro for other_29497 (other)
macro_rules! Depcrate_um_ktmw32other_29497 {
() => {
// Module: crate::um::ktmw32
// Provides: {"other_29497"}
// Dependencies: {}
extern "system" { pub fn CreateTransaction (lpTransactionAttributes : LPSECURITY_ATTRIBUTES , UOW : LPGUID , CreateOptions : DWORD , IsolationLevel : DWORD , IsolationFlags : DWORD , Timeout : DWORD , Description : LPWSTR ,) -> HANDLE ; pub fn CommitTransaction (TransactionHandle : HANDLE ,) -> BOOL ; pub fn RollbackTransaction (TransactionHandle : HANDLE ,) -> BOOL ; }
};
}
