macro_rules! deps {
    () => {
        Codes!();
    };
}

macro_rules! Table {
    () => {
        deps!();
        # [derive (Default , Clone , Copy)] struct Table { codes : Codes , bits : usize , }
    };
}

Table!()