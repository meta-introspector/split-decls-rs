macro_rules! AssociatedFunctionArguments {
    () => {
        struct AssociatedFunctionArguments < 'syn > { self_token : Option < & 'syn syn :: token :: SelfValue > , db_ty : & 'syn syn :: Type , db_ident : & 'syn syn :: Ident , db_lt : Option < & 'syn syn :: Lifetime > , input_ids : Vec < syn :: Ident > , input_tys : Vec < & 'syn syn :: Type > , output_ty : syn :: Type , }
    };
}

AssociatedFunctionArguments!()