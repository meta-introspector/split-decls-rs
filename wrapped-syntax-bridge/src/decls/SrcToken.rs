macro_rules! SrcToken {
    () => {
        trait SrcToken < Ctx , S > { fn kind (& self , ctx : & Ctx) -> SyntaxKind ; fn to_char (& self , ctx : & Ctx) -> Option < char > ; fn to_text (& self , ctx : & Ctx) -> SmolStr ; fn as_leaf (& self) -> Option < & tt :: Leaf < S > > { None } }
    };
}

SrcToken!()