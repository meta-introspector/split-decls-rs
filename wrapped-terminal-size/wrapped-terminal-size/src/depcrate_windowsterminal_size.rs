// Generated macro for terminal_size (function)
macro_rules! Depcrate_windowsterminal_size {
() => {
// Module: crate::windows
// Provides: {"terminal_size"}
// Dependencies: {}
# [doc = " Returns the size of the terminal."] # [doc = ""] # [doc = " This function checks the stdout, stderr, and stdin streams (in that order)."] # [doc = " The size of the first stream that is a TTY will be returned.  If nothing"] # [doc = " is a TTY, then `None` is returned."] # [doc = ""] # [doc = " Note that this returns the size of the actual command window, and"] # [doc = " not the overall size of the command window buffer"] pub fn terminal_size () -> Option < (Width , Height) > { use windows_sys :: Win32 :: System :: Console :: { GetStdHandle , STD_ERROR_HANDLE , STD_INPUT_HANDLE , STD_OUTPUT_HANDLE , } ; if let Some (size) = terminal_size_of (unsafe { BorrowedHandle :: borrow_raw (GetStdHandle (STD_OUTPUT_HANDLE) as RawHandle) }) { Some (size) } else if let Some (size) = terminal_size_of (unsafe { BorrowedHandle :: borrow_raw (GetStdHandle (STD_ERROR_HANDLE) as RawHandle) }) { Some (size) } else if let Some (size) = terminal_size_of (unsafe { BorrowedHandle :: borrow_raw (GetStdHandle (STD_INPUT_HANDLE) as RawHandle) }) { Some (size) } else { None } }
};
}
