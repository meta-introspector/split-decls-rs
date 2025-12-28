macro_rules! deps {
    () => {
        Transparency!();
        SyntaxContext!();
        ExpnId!();
        Symbol!();
    };
}

macro_rules! SyntaxContextData {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug)] struct SyntaxContextData { # [doc = " The last macro expansion in the chain."] # [doc = " (Here we say the most deeply nested macro expansion is the \"outermost\" expansion.)"] outer_expn : ExpnId , # [doc = " Transparency of the last macro expansion"] outer_transparency : Transparency , parent : SyntaxContext , # [doc = " This context, but with all transparent and semi-opaque expansions filtered away."] opaque : SyntaxContext , # [doc = " This context, but with all transparent expansions filtered away."] opaque_and_semiopaque : SyntaxContext , # [doc = " Name of the crate to which `$crate` with this context would resolve."] dollar_crate_name : Symbol , }
    };
}

SyntaxContextData!();