macro_rules! deps {
    () => {
        CycleUsage!();
        Cycle!();
        StackCount!();
        CycleStack!();
        CycleError!();
        Alias!();
    };
}

macro_rules! report_cycle {
    () => {
        deps!();
        # [inline (never)] # [cold] pub fn report_cycle < 'a > (sess : & 'a Session , CycleError { usage , cycle : stack } : & CycleError ,) -> Diag < 'a > { assert ! (! stack . is_empty ()) ; let span = stack [0] . query . info . default_span (stack [1 % stack . len ()] . span) ; let mut cycle_stack = Vec :: new () ; use crate :: error :: StackCount ; let stack_count = if stack . len () == 1 { StackCount :: Single } else { StackCount :: Multiple } ; for i in 1 .. stack . len () { let query = & stack [i] . query ; let span = query . info . default_span (stack [(i + 1) % stack . len ()] . span) ; cycle_stack . push (CycleStack { span , desc : query . info . description . to_owned () }) ; } let mut cycle_usage = None ; if let Some ((span , ref query)) = * usage { cycle_usage = Some (crate :: error :: CycleUsage { span : query . info . default_span (span) , usage : query . info . description . to_string () , }) ; } let alias = if stack . iter () . all (| entry | matches ! (entry . query . info . def_kind , Some (DefKind :: TyAlias))) { Some (crate :: error :: Alias :: Ty) } else if stack . iter () . all (| entry | entry . query . info . def_kind == Some (DefKind :: TraitAlias)) { Some (crate :: error :: Alias :: Trait) } else { None } ; let cycle_diag = crate :: error :: Cycle { span , cycle_stack , stack_bottom : stack [0] . query . info . description . to_owned () , alias , cycle_usage , stack_count , note_span : () , } ; sess . dcx () . create_err (cycle_diag) }
    };
}

report_cycle!();