macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! Bin {
    () => {
        deps!();
        # [derive (Serialize , Debug)] pub (crate) struct Bin { pub name : Name , pub path : PathBuf , }
    };
}

Bin!()