macro_rules! DefaultKind {
    () => {
        # [derive (Clone , Copy , Debug)] enum DefaultKind { Ignore , # [cfg (not (windows))] Stop , Term , }
    };
}

DefaultKind!();