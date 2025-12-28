macro_rules! DepKind {
    () => {
        # [doc = " This serves as an index into arrays built by `make_dep_kind_array`."] # [derive (Clone , Copy , PartialEq , Eq , Hash)] pub struct DepKind { variant : u16 , }
    };
}

DepKind!();