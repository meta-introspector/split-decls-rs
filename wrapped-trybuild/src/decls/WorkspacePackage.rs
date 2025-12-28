macro_rules! deps {
    () => {
        Edition!();
    };
}

macro_rules! WorkspacePackage {
    () => {
        deps!();
        # [derive (Deserialize , Default , Debug)] pub (crate) struct WorkspacePackage { pub edition : Option < Edition > , }
    };
}

WorkspacePackage!()