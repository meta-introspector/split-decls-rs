macro_rules! deps {
    () => {
        Height!();
        Width!();
    };
}

macro_rules! terminal_size_using_handle {
    () => {
        deps!();
        # [doc = " Returns the size of the terminal using the given handle, if available."] # [doc = ""] # [doc = " The given handle must be an open handle."] # [doc = ""] # [doc = " If the given handle is not a tty, returns `None`"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `handle` must be a valid open file handle."] # [deprecated (note = "Use `terminal_size_of` instead.
     Use `BorrowedHandle::borrow_raw` to convert a raw handle into a `BorrowedHandle` if needed.")] pub unsafe fn terminal_size_using_handle (handle : RawHandle) -> Option < (Width , Height) > { terminal_size_of (BorrowedHandle :: borrow_raw (handle)) }
    };
}

terminal_size_using_handle!();