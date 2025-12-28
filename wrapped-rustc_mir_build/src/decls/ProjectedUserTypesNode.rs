macro_rules! deps {
    () => {
        ProjectedUserTypesOp!();
    };
}

macro_rules! ProjectedUserTypesNode {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) enum ProjectedUserTypesNode < 'a > { None , Chain { parent : & 'a Self , op : ProjectedUserTypesOp } , }
    };
}

ProjectedUserTypesNode!();