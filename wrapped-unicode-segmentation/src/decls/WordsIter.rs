macro_rules! deps {
    () => {
        UnicodeWordsIter!();
        AsciiWordsIter!();
    };
}

macro_rules! WordsIter {
    () => {
        deps!();
        # [derive (Debug)] enum WordsIter < 'a > { Ascii (AsciiWordsIter < 'a >) , Unicode (UnicodeWordsIter < 'a >) , }
    };
}

WordsIter!();