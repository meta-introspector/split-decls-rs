macro_rules! deps {
    () => {
        SanitizerSet!();
    };
}

macro_rules! macro_508 {
    () => {
        deps!();
        rustc_data_structures :: external_bitflags_debug ! { SanitizerSet }
    };
}

macro_508!()