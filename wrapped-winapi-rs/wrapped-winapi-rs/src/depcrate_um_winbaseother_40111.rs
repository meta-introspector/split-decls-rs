// Generated macro for other_40111 (other)
macro_rules! Depcrate_um_winbaseother_40111 {
() => {
// Module: crate::um::winbase
// Provides: {"other_40111"}
// Dependencies: {}
extern "system" { pub fn LoadModule (lpModuleName : LPCSTR , lpParameterBlock : LPVOID ,) -> DWORD ; pub fn WinExec (lpCmdLine : LPCSTR , uCmdShow : UINT ,) -> UINT ; pub fn SetTapePosition (hDevice : HANDLE , dwPositionMethod : DWORD , dwPartition : DWORD , dwOffsetLow : DWORD , dwOffsetHigh : DWORD , bImmediate : BOOL ,) -> DWORD ; pub fn GetTapePosition (hDevice : HANDLE , dwPositionType : DWORD , lpdwPartition : LPDWORD , lpdwOffsetLow : LPDWORD , lpdwOffsetHigh : LPDWORD ,) -> DWORD ; pub fn PrepareTape (hDevice : HANDLE , dwOperation : DWORD , bImmediate : BOOL ,) -> DWORD ; pub fn EraseTape (hDevice : HANDLE , dwEraseType : DWORD , bImmediate : BOOL ,) -> DWORD ; pub fn CreateTapePartition (hDevice : HANDLE , dwPartitionMethod : DWORD , dwCount : DWORD , dwSize : DWORD ,) -> DWORD ; pub fn WriteTapemark (hDevice : HANDLE , dwTapemarkType : DWORD , dwTapemarkCount : DWORD , bImmediate : BOOL ,) -> DWORD ; pub fn GetTapeStatus (hDevice : HANDLE ,) -> DWORD ; pub fn GetTapeParameters (hDevice : HANDLE , dwOperation : DWORD , lpdwSize : LPDWORD , lpTapeInformation : LPVOID ,) -> DWORD ; pub fn SetTapeParameters (hDevice : HANDLE , dwOperation : DWORD , lpTapeInformation : LPVOID ,) -> DWORD ; pub fn MulDiv (nNumber : c_int , nNumerator : c_int , nDenominator : c_int ,) -> c_int ; }
};
}
