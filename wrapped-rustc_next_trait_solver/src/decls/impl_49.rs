macro_rules! deps {
    () => {
        Ambiguous!();
        ReplaceProjectionWith!();
        SolverDelegate!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < D , I > ReplaceProjectionWith < '_ , '_ , I , D > where D : SolverDelegate < Interner = I > , I : Interner , { fn projection_may_match (& mut self , source_projection : ty :: Binder < I , ty :: ProjectionPredicate < I > > , target_projection : ty :: AliasTerm < I > ,) -> bool { source_projection . item_def_id () == target_projection . def_id && self . ecx . probe (| _ | ProbeKind :: ProjectionCompatibility) . enter (| ecx | -> Result < _ , NoSolution > { let source_projection = ecx . instantiate_binder_with_infer (source_projection) ; ecx . eq (self . param_env , source_projection . projection_term , target_projection) ? ; ecx . try_evaluate_added_goals () }) . is_ok () } # [doc = " Try to replace an alias with the term present in the projection bounds of the self type."] # [doc = " Returns `Ok<None>` if this alias is not eligible to be replaced, or bail with"] # [doc = " `Err(Ambiguous)` if it's uncertain which projection bound to replace the term with due"] # [doc = " to multiple bounds applying."] fn try_eagerly_replace_alias (& mut self , alias_term : ty :: AliasTerm < I > ,) -> Result < Option < I :: Term > , Ambiguous > { if alias_term . self_ty () != self . self_ty { return Ok (None) ; } let Some (replacements) = self . mapping . get (& alias_term . def_id) else { return Ok (None) ; } ; let mut matching_projections = replacements . iter () . filter (| source_projection | self . projection_may_match (* * source_projection , alias_term)) ; let Some (replacement) = matching_projections . next () else { panic ! ("could not replace {alias_term:?} with term from from {:?}" , self . self_ty) ; } ; if matching_projections . next () . is_some () { return Err (Ambiguous) ; } let replacement = self . ecx . instantiate_binder_with_infer (* replacement) ; self . nested . extend (self . ecx . eq_and_get_goals (self . param_env , alias_term , replacement . projection_term) . expect ("expected to be able to unify goal projection with dyn's projection") ,) ; Ok (Some (replacement . term)) } }
    };
}

impl_49!();