macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! test {
    () => {
        deps!();
        # [cfg (test)] mod test { use super :: * ; # [test] fn test_record_backwards_compat () { Span :: current () . record ("some-key" , "some text") ; Span :: current () . record ("some-key" , false) ; } }
    };
}

test!()