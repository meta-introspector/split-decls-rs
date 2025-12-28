macro_rules! deps {
    () => {
        ParseCtxt!();
    };
}

macro_rules! parse_by_kind {
    () => {
        deps!();
        # [doc = " Helper macro for parsing custom MIR."] # [doc = ""] # [doc = " Example usage looks something like:"] # [doc = " ```rust,ignore (incomplete example)"] # [doc = " parse_by_kind!("] # [doc = "     self, // : &ParseCtxt"] # [doc = "     expr_id, // what you're matching against"] # [doc = "     \"assignment\", // the thing you're trying to parse"] # [doc = "     @call(\"mir_assign\", args) => { args[0] }, // match invocations of the `mir_assign` special function"] # [doc = "     ExprKind::Assign { lhs, .. } => { lhs }, // match thir assignment expressions"] # [doc = "     // no need for fallthrough case - reasonable error is automatically generated"] # [doc = " )"] # [doc = " ```"] macro_rules ! parse_by_kind { ($ self : ident , $ expr_id : expr , $ expr_name : pat , $ expected : literal , $ (@ call ($ name : ident , $ args : ident) => $ call_expr : expr ,) * $ (@ variant ($ adt : ident , $ variant : ident) => $ variant_expr : expr ,) * $ ($ pat : pat $ (if $ guard : expr) ? => $ expr : expr ,) *) => { { let expr_id = $ self . preparse ($ expr_id) ; let expr = &$ self . thir [expr_id] ; tracing :: debug ! ("Trying to parse {:?} as {}" , expr . kind , $ expected) ; let $ expr_name = expr ; match & expr . kind { $ (ExprKind :: Call { ty , fun : _ , args : $ args , .. } if { match ty . kind () { ty :: FnDef (did , _) => { $ self . tcx . is_diagnostic_item (rustc_span :: sym ::$ name , * did) } _ => false , } } => $ call_expr ,) * $ (ExprKind :: Adt (box AdtExpr { adt_def , variant_index , .. }) if { $ self . tcx . is_diagnostic_item (rustc_span :: sym ::$ adt , adt_def . did ()) && adt_def . variants () [* variant_index] . name == rustc_span :: sym ::$ variant } => $ variant_expr ,) * $ ($ pat $ (if $ guard) ? => $ expr ,) * # [allow (unreachable_patterns)] _ => return Err ($ self . expr_error (expr_id , $ expected)) } } } ; }
    };
}

parse_by_kind!()