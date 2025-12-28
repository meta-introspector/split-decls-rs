macro_rules! deps {
    () => {
        DataFormat!();
    };
}

macro_rules! FilterSet {
    () => {
        deps!();
        # [derive (Copy , Clone , Default , Debug , PartialEq , Eq)] pub (crate) struct FilterSet { flags : usize , against : Option < DataFormat > , }
    };
}

FilterSet!()