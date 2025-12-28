macro_rules! InitLocation {
    () => {
        # [doc = " Initializations can be from an argument or from a statement. Arguments"] # [doc = " do not have locations, in those cases the `Local` is kept.."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum InitLocation { Argument (Local) , Statement (Location) , }
    };
}

InitLocation!()