macro_rules! SeekTarget {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq)] enum SeekTarget { BlockEntry (BasicBlock) , Early (Location) , After (Location) , }
    };
}

SeekTarget!()