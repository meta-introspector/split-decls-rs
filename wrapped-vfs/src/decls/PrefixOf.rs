macro_rules! PrefixOf {
    () => {
        # [doc = " Implements [`fst::Automaton`]"] # [doc = ""] # [doc = " It will match if `prefix_of` is a prefix of the given data."] struct PrefixOf < 'a > { prefix_of : & 'a [u8] , }
    };
}

PrefixOf!();