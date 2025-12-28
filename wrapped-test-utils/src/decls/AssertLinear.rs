macro_rules! deps {
    () => {
        Round!();
    };
}

macro_rules! AssertLinear {
    () => {
        deps!();
        # [derive (Default)] pub struct AssertLinear { rounds : Vec < Round > , }
    };
}

AssertLinear!();