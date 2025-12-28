macro_rules! deps {
    () => {
        AsciiIndicesIter!();
        UnicodeIndicesIter!();
    };
}

macro_rules! IndicesIter {
    () => {
        deps!();
        # [derive (Debug)] enum IndicesIter < 'a > { Ascii (AsciiIndicesIter < 'a >) , Unicode (UnicodeIndicesIter < 'a >) , }
    };
}

IndicesIter!();