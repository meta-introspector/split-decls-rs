macro_rules! deps {
    () => {
        Redactions!();
    };
}

macro_rules! NormalizeToExpected {
    () => {
        deps!();
        # [doc = " Adjust `actual` based on `expected`"] pub struct NormalizeToExpected < 'a > { substitutions : Option < & 'a Redactions > , unordered : bool , }
    };
}

NormalizeToExpected!();