macro_rules! deps {
    () => {
        RelateResult!();
        Interner!();
        Relate!();
        ExpectedFound!();
        FnSig!();
        TypeRelation!();
        VarianceDiagInfo!();
        TypeError!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < I : Interner > Relate < I > for ty :: FnSig < I > { fn relate < R : TypeRelation < I > > (relation : & mut R , a : ty :: FnSig < I > , b : ty :: FnSig < I > ,) -> RelateResult < I , ty :: FnSig < I > > { let cx = relation . cx () ; if a . c_variadic != b . c_variadic { return Err (TypeError :: VariadicMismatch ({ let a = a . c_variadic ; let b = b . c_variadic ; ExpectedFound :: new (a , b) })) ; } if a . safety != b . safety { return Err (TypeError :: SafetyMismatch (ExpectedFound :: new (a . safety , b . safety))) ; } if a . abi != b . abi { return Err (TypeError :: AbiMismatch (ExpectedFound :: new (a . abi , b . abi))) ; } ; let a_inputs = a . inputs () ; let b_inputs = b . inputs () ; if a_inputs . len () != b_inputs . len () { return Err (TypeError :: ArgCount) ; } let inputs_and_output = iter :: zip (a_inputs . iter () , b_inputs . iter ()) . map (| (a , b) | ((a , b) , false)) . chain (iter :: once (((a . output () , b . output ()) , true))) . map (| ((a , b) , is_output) | { if is_output { relation . relate (a , b) } else { relation . relate_with_variance (ty :: Contravariant , VarianceDiagInfo :: default () , a , b ,) } }) . enumerate () . map (| (i , r) | match r { Err (TypeError :: Sorts (exp_found) | TypeError :: ArgumentSorts (exp_found , _)) => { Err (TypeError :: ArgumentSorts (exp_found , i)) } Err (TypeError :: Mutability | TypeError :: ArgumentMutability (_)) => { Err (TypeError :: ArgumentMutability (i)) } r => r , }) ; Ok (ty :: FnSig { inputs_and_output : cx . mk_type_list_from_iter (inputs_and_output) ? , c_variadic : a . c_variadic , safety : a . safety , abi : a . abi , }) } }
    };
}

impl_117!();