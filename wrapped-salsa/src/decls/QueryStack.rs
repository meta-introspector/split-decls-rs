macro_rules! deps {
    () => {
        ActiveQuery!();
    };
}

macro_rules! QueryStack {
    () => {
        deps!();
        # [derive (Default)] pub (crate) struct QueryStack { stack : Vec < ActiveQuery > , len : usize , }
    };
}

QueryStack!();