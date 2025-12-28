macro_rules! deps {
    () => {
        FSETable!();
    };
}

macro_rules! FseTableMode {
    () => {
        deps!();
        # [derive (Clone)] # [allow (clippy :: large_enum_variant)] enum FseTableMode < 'a > { Predefined (& 'a FSETable) , Encoded (FSETable) , RepeateLast (& 'a FSETable) , }
    };
}

FseTableMode!()