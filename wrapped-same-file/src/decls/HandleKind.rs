macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! HandleKind {
    () => {
        deps!();
        # [derive (Debug)] enum HandleKind { # [doc = " Used when opening a file or acquiring ownership of a file."] Owned (winutil :: Handle) , # [doc = " Used for stdio."] Borrowed (winutil :: HandleRef) , }
    };
}

HandleKind!()