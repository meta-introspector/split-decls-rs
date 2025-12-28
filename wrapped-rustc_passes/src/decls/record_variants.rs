macro_rules! record_variants {
    () => {
        macro_rules ! record_variants { (($ self : ident , $ val : expr , $ kind : expr , $ id : expr , $ mod : ident , $ ty : ty , $ tykind : ident) , [$ ($ variant : ident) ,*]) => { match $ kind { $ ($ mod ::$ tykind ::$ variant { .. } => { $ self . record_variant (stringify ! ($ ty) , stringify ! ($ variant) , $ id , $ val) }) * } } ; }
    };
}

record_variants!();