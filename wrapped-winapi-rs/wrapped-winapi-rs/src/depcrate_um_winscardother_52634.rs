// Generated macro for other_52634 (other)
macro_rules! Depcrate_um_winscardother_52634 {
() => {
// Module: crate::um::winscard
// Provides: {"other_52634"}
// Dependencies: {}
extern "system" { pub fn SCardLocateCardsByATRA (hContext : SCARDCONTEXT , rgAtrMasks : LPSCARD_ATRMASK , cAtrs : DWORD , rgReaderStates : LPSCARD_READERSTATEA , cReaders : DWORD ,) -> LONG ; pub fn SCardLocateCardsByATRW (hContext : SCARDCONTEXT , rgAtrMasks : LPSCARD_ATRMASK , cAtrs : DWORD , rgReaderStates : LPSCARD_READERSTATEW , cReaders : DWORD ,) -> LONG ; pub fn SCardGetStatusChangeA (hContext : SCARDCONTEXT , dwTimeout : DWORD , rgReaderStates : LPSCARD_READERSTATEA , cReaders : DWORD ,) -> LONG ; pub fn SCardGetStatusChangeW (hContext : SCARDCONTEXT , dwTimeout : DWORD , rgReaderStates : LPSCARD_READERSTATEW , cReaders : DWORD ,) -> LONG ; pub fn SCardCancel (hContext : SCARDCONTEXT ,) -> LONG ; }
};
}
