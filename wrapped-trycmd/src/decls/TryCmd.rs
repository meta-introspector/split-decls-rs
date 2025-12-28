macro_rules! deps {
    () => {
        Filesystem!();
        Step!();
    };
}

macro_rules! TryCmd {
    () => {
        deps!();
        # [derive (Clone , Default , Debug , PartialEq , Eq)] pub (crate) struct TryCmd { pub (crate) steps : Vec < Step > , pub (crate) fs : Filesystem , }
    };
}

TryCmd!()