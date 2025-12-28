macro_rules! deps {
    () => {
        AsciiWordBoundIter!();
    };
}

macro_rules! AsciiIndicesIter {
    () => {
        deps!();
        type AsciiIndicesIter < 'a > = Filter < AsciiWordBoundIter < 'a > , fn (& (usize , & 'a str)) -> bool > ;
    };
}

AsciiIndicesIter!()