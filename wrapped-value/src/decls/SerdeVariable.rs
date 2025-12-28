macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! SerdeVariable {
    () => {
        deps!();
        # [derive (Debug)] struct SerdeVariable (Name) ;
    };
}

SerdeVariable!();