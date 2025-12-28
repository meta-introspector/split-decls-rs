macro_rules! bigrams {
    () => {
        # [doc = " Returns an Iterator of char tuples."] fn bigrams (s : & str) -> impl Iterator < Item = (char , char) > + '_ { s . chars () . zip (s . chars () . skip (1)) }
    };
}

bigrams!()