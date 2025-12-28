macro_rules! ErrorKind {
    () => {
        # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Ord , PartialOrd)] pub enum ErrorKind { Interrupted , UnexpectedEof , WouldBlock , Other , WriteAllEof , }
    };
}

ErrorKind!();