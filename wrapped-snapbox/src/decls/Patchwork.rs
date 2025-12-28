macro_rules! deps {
    () => {
        OrdRange!();
    };
}

macro_rules! Patchwork {
    () => {
        deps!();
        # [derive (Debug)] struct Patchwork { text : String , indels : BTreeMap < OrdRange , (usize , String) > , }
    };
}

Patchwork!()