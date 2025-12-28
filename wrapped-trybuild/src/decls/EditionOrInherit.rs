macro_rules! deps {
    () => {
        Edition!();
    };
}

macro_rules! EditionOrInherit {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) enum EditionOrInherit { Edition (Edition) , Inherit , }
    };
}

EditionOrInherit!();