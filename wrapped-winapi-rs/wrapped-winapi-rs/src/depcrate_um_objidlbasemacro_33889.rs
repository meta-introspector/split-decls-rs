// Generated macro for macro_33889 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33889 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33889"}
// Dependencies: {}
RIDL ! { # [uuid (0x0000000c , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IStream (IStreamVtbl) : ISequentialStream (ISequentialStreamVtbl) { fn Seek (dlibMove : LARGE_INTEGER , dwOrigin : DWORD , plibNewPosition : * mut ULARGE_INTEGER ,) -> HRESULT , fn SetSize (libNewSize : ULARGE_INTEGER ,) -> HRESULT , fn CopyTo (pstm : * mut IStream , cb : ULARGE_INTEGER , pcbRead : * mut ULARGE_INTEGER , pcbWritten : * mut ULARGE_INTEGER ,) -> HRESULT , fn Commit (grfCommitFlags : DWORD ,) -> HRESULT , fn Revert () -> HRESULT , fn LockRegion (libOffset : ULARGE_INTEGER , cb : ULARGE_INTEGER , dwLockType : DWORD ,) -> HRESULT , fn UnlockRegion (libOffset : ULARGE_INTEGER , cb : ULARGE_INTEGER , dwLockType : DWORD ,) -> HRESULT , fn Stat (pstatstg : * mut STATSTG , grfStatFlag : DWORD ,) -> HRESULT , fn Clone (ppstm : * mut * mut IStream ,) -> HRESULT , } }
};
}
