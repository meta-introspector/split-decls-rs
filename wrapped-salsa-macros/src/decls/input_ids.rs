macro_rules! deps {
    () => {
        Hygiene!();
    };
}

macro_rules! input_ids {
    () => {
        deps!();
        # [doc = " Returns a vector of ids representing the function arguments."] # [doc = " Prefers to reuse the names given by the user, if possible."] pub fn input_ids (hygiene : & Hygiene , sig : & syn :: Signature , skip : usize) -> Vec < syn :: Ident > { sig . inputs . iter () . skip (skip) . zip (0 ..) . map (| (input , index) | { if let syn :: FnArg :: Typed (typed) = input { if let syn :: Pat :: Ident (ident) = & * typed . pat { return ident . ident . clone () ; } } hygiene . ident (format ! ("input{index}")) }) . collect () }
    };
}

input_ids!();