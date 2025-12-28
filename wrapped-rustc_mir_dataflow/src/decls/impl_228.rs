macro_rules! deps {
    () => {
        PeekCall!();
        PeekCallKind!();
        PeekMustBeNotTemporary!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl PeekCall { fn from_terminator < 'tcx > (tcx : TyCtxt < 'tcx > , terminator : & mir :: Terminator < 'tcx > ,) -> Option < Self > { use mir :: Operand ; let span = terminator . source_info . span ; if let mir :: TerminatorKind :: Call { func : Operand :: Constant (func) , args , .. } = & terminator . kind && let ty :: FnDef (def_id , fn_args) = * func . const_ . ty () . kind () { if tcx . intrinsic (def_id) ? . name != sym :: rustc_peek { return None ; } assert_eq ! (fn_args . len () , 1) ; let kind = PeekCallKind :: from_arg_ty (fn_args . type_at (0)) ; let arg = match & args [0] . node { Operand :: Copy (place) | Operand :: Move (place) => { if let Some (local) = place . as_local () { local } else { tcx . dcx () . emit_err (PeekMustBeNotTemporary { span }) ; return None ; } } _ => { tcx . dcx () . emit_err (PeekMustBeNotTemporary { span }) ; return None ; } } ; return Some (PeekCall { arg , kind , span }) ; } None } }
    };
}

impl_228!();