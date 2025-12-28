macro_rules! deps {
    () => {
        AsciiWordBoundIter!();
    };
}

macro_rules! AsciiWordsIter {
    () => {
        deps!();
        type AsciiWordsIter < 'a > = Filter < core :: iter :: Map < AsciiWordBoundIter < 'a > , fn ((usize , & 'a str)) -> & 'a str > , fn (& & 'a str) -> bool , > ;
    };
}

AsciiWordsIter!()