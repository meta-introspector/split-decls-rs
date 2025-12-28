macro_rules! op_trait_as_method_name {
    () => {
        fn op_trait_as_method_name (tcx : TyCtxt < '_ > , trait_did : DefId) -> Option < & 'static str > { let m = match tcx . as_lang_item (trait_did) ? { LangItem :: Add => "add" , LangItem :: Sub => "sub" , LangItem :: Mul => "mul" , LangItem :: Div => "div" , LangItem :: Rem => "rem" , LangItem :: Neg => "neg" , LangItem :: Not => "not" , LangItem :: BitXor => "bitxor" , LangItem :: BitAnd => "bitand" , LangItem :: BitOr => "bitor" , LangItem :: Shl => "shl" , LangItem :: Shr => "shr" , LangItem :: AddAssign => "add_assign" , LangItem :: SubAssign => "sub_assign" , LangItem :: MulAssign => "mul_assign" , LangItem :: DivAssign => "div_assign" , LangItem :: RemAssign => "rem_assign" , LangItem :: BitXorAssign => "bitxor_assign" , LangItem :: BitAndAssign => "bitand_assign" , LangItem :: BitOrAssign => "bitor_assign" , LangItem :: ShlAssign => "shl_assign" , LangItem :: ShrAssign => "shr_assign" , LangItem :: Index => "index" , LangItem :: IndexMut => "index_mut" , _ => return None , } ; Some (m) }
    };
}

op_trait_as_method_name!();