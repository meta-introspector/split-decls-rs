macro_rules! deps {
    () => {
        BigEndian!();
        LittleEndian!();
    };
}

macro_rules! private {
    () => {
        deps!();
        mod private { pub trait Sealed { } impl Sealed for super :: BigEndian { } impl Sealed for super :: LittleEndian { } }
    };
}

private!();