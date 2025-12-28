macro_rules! emit_pairs {
    () => {
        macro_rules ! emit_pairs { ($ self : ident $ modifier : ident , $ ($ pair : ident $ name : literal $ hi : literal $ lo : literal ,) *) => { match ($ self , $ modifier) { $ ((AvrInlineAsmReg ::$ pair , Some ('h')) => $ hi , (AvrInlineAsmReg ::$ pair , Some ('l')) => $ lo , (AvrInlineAsmReg ::$ pair , _) => $ name ,) * _ => $ self . name () , } } ; }
    };
}

emit_pairs!();