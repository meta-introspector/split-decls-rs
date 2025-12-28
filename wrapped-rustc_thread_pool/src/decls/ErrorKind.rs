macro_rules! ErrorKind {
    () => {
        # [derive (Debug)] enum ErrorKind { GlobalPoolAlreadyInitialized , IOError (io :: Error) , }
    };
}

ErrorKind!();