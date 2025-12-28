macro_rules! deps {
    () => {
        UWordBounds!();
    };
}

macro_rules! UnicodeWordsIter {
    () => {
        deps!();
        type UnicodeWordsIter < 'a > = Filter < UWordBounds < 'a > , fn (& & 'a str) -> bool > ;
    };
}

UnicodeWordsIter!();