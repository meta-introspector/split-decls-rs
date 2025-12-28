macro_rules! OsKind {
    () => {
        # [derive (Debug , PartialEq , Copy , Clone)] enum OsKind { Windows , VEXos , Other , }
    };
}

OsKind!()