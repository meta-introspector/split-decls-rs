macro_rules! QueryStackFrameExtra {
    () => {
        # [derive (Clone , Debug)] pub struct QueryStackFrameExtra { pub description : String , span : Option < Span > , pub def_kind : Option < DefKind > , }
    };
}

QueryStackFrameExtra!()