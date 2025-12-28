macro_rules! Encoding {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [repr (u8)] pub enum Encoding { LiteralString = crate :: lexer :: APOSTROPHE , BasicString = crate :: lexer :: QUOTATION_MARK , MlLiteralString = 1 , MlBasicString , }
    };
}

Encoding!();