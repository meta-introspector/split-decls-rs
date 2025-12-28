macro_rules! deps {
    () => {
        Case!();
    };
}

macro_rules! Runner {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct Runner { cases : Vec < Case > , }
    };
}

Runner!();