macro_rules! deps {
    () => {
        Encoding!();
        Source!();
        Span!();
    };
}

macro_rules! Raw {
    () => {
        deps!();
        # [doc = " A slice of [`Source`]"] # [derive (Copy , Clone , Debug)] pub struct Raw < 'i > { raw : & 'i str , encoding : Option < Encoding > , span : Span , }
    };
}

Raw!()