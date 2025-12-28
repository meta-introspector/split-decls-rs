macro_rules! deps {
    () => {
        UWordBoundIndices!();
    };
}

macro_rules! UnicodeIndicesIter {
    () => {
        deps!();
        type UnicodeIndicesIter < 'a > = Filter < UWordBoundIndices < 'a > , fn (& (usize , & 'a str)) -> bool > ;
    };
}

UnicodeIndicesIter!();