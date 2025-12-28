macro_rules! deps {
    () => {
        UnicodeIndicesIter!();
        AsciiIndicesIter!();
    };
}

macro_rules! IndicesIter {
    () => {
        deps!();
        # [derive (Debug)] enum IndicesIter < 'a > { Ascii (AsciiIndicesIter < 'a >) , Unicode (UnicodeIndicesIter < 'a >) , }
    };
}

IndicesIter!()