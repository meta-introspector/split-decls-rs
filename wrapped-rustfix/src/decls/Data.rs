macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! Data {
    () => {
        deps!();
        # [doc = " A container that allows easily replacing chunks of its data."] # [derive (Debug , Clone , Default)] pub struct Data { # [doc = " Original data."] original : Vec < u8 > , # [doc = " [`Span`]s covering the full range of the original data."] # [doc = " Important: it's expected that the underlying implementation maintains this in order,"] # [doc = " sorted ascending by start position."] parts : Vec < Span > , }
    };
}

Data!();