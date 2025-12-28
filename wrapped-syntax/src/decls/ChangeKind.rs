macro_rules! ChangeKind {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] enum ChangeKind { Insert , ReplaceRange , Replace , }
    };
}

ChangeKind!();