macro_rules! deps {
    () => {
        SyntaxContext!();
    };
}

macro_rules! HygieneDecodeContext {
    () => {
        deps!();
        # [doc = " Additional information used to assist in decoding hygiene data"] # [derive (Default)] pub struct HygieneDecodeContext { remapped_ctxts : Lock < IndexVec < u32 , Option < SyntaxContext > > > , }
    };
}

HygieneDecodeContext!()