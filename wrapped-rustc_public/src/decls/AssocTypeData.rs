macro_rules! deps {
    () => {
        Symbol!();
        ImplTraitInTraitData!();
    };
}

macro_rules! AssocTypeData {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Debug , Eq , Serialize)] pub enum AssocTypeData { Normal (Symbol) , # [doc = " The associated type comes from an RPITIT. It has no name, and the"] # [doc = " `ImplTraitInTraitData` provides additional information about its"] # [doc = " source."] Rpitit (ImplTraitInTraitData) , }
    };
}

AssocTypeData!();