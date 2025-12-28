macro_rules! deps {
    () => {
        RelateResult!();
        ConstKind!();
        Const!();
        UnevaluatedConst!();
        ExpectedFound!();
        Interner!();
        VarianceDiagInfo!();
        TypeError!();
        TypeRelation!();
    };
}

macro_rules! structurally_relate_consts {
    () => {
        deps!();
        # [doc = " Relates `a` and `b` structurally, calling the relation for all nested values."] # [doc = " Any semantic equality, e.g. of unevaluated consts, and inference variables have"] # [doc = " to be handled by the caller."] # [doc = ""] # [doc = " FIXME: This is not totally structural, which probably should be fixed."] # [doc = " See the HACKs below."] pub fn structurally_relate_consts < I : Interner , R : TypeRelation < I > > (relation : & mut R , mut a : I :: Const , mut b : I :: Const ,) -> RelateResult < I , I :: Const > { trace ! ("structurally_relate_consts::<{}>(a = {:?}, b = {:?})" , std :: any :: type_name ::< R > () , a , b) ; let cx = relation . cx () ; if cx . features () . generic_const_exprs () { a = cx . expand_abstract_consts (a) ; b = cx . expand_abstract_consts (b) ; } trace ! ("structurally_relate_consts::<{}>(normed_a = {:?}, normed_b = {:?})" , std :: any :: type_name ::< R > () , a , b) ; let is_match = match (a . kind () , b . kind ()) { (ty :: ConstKind :: Infer (_) , _) | (_ , ty :: ConstKind :: Infer (_)) => { panic ! ("var types encountered in structurally_relate_consts: {:?} {:?}" , a , b) } (ty :: ConstKind :: Error (_) , _) => return Ok (a) , (_ , ty :: ConstKind :: Error (_)) => return Ok (b) , (ty :: ConstKind :: Param (a_p) , ty :: ConstKind :: Param (b_p)) if a_p . index () == b_p . index () => { true } (ty :: ConstKind :: Placeholder (p1) , ty :: ConstKind :: Placeholder (p2)) => p1 == p2 , (ty :: ConstKind :: Value (a_val) , ty :: ConstKind :: Value (b_val)) => { a_val . valtree () == b_val . valtree () } (ty :: ConstKind :: Unevaluated (au) , ty :: ConstKind :: Unevaluated (bu)) if au . def == bu . def => { if cfg ! (debug_assertions) { let a_ty = cx . type_of (au . def) . instantiate (cx , au . args) ; let b_ty = cx . type_of (bu . def) . instantiate (cx , bu . args) ; assert_eq ! (a_ty , b_ty) ; } let args = relation . relate_with_variance (ty :: Invariant , VarianceDiagInfo :: default () , au . args , bu . args ,) ? ; return Ok (Const :: new_unevaluated (cx , ty :: UnevaluatedConst { def : au . def , args })) ; } (ty :: ConstKind :: Expr (ae) , ty :: ConstKind :: Expr (be)) => { let expr = relation . relate (ae , be) ? ; return Ok (Const :: new_expr (cx , expr)) ; } _ => false , } ; if is_match { Ok (a) } else { Err (TypeError :: ConstMismatch (ExpectedFound :: new (a , b))) } }
    };
}

structurally_relate_consts!();