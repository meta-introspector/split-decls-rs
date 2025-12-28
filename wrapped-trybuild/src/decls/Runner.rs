macro_rules! deps {
    () => {
        Test!();
    };
}

macro_rules! Runner {
    () => {
        deps!();
        # [derive (Debug)] struct Runner { tests : Vec < Test > , }
    };
}

Runner!()