macro_rules! deps {
    () => {
        PatCtxt!();
        NonEmptyNeverPattern!();
    };
}

macro_rules! check_never_pattern {
    () => {
        deps!();
        # [doc = " Check that never patterns are only used on inhabited types."] fn check_never_pattern < 'tcx > (cx : & PatCtxt < '_ , 'tcx > , pat : & Pat < 'tcx > ,) -> Result < () , ErrorGuaranteed > { if let PatKind :: Never = pat . kind { if ! cx . is_uninhabited (pat . ty) { return Err (cx . tcx . dcx () . emit_err (NonEmptyNeverPattern { span : pat . span , ty : pat . ty })) ; } } Ok (()) }
    };
}

check_never_pattern!();