macro_rules! impl_spans {
    () => {
        fn impl_spans (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> impl Iterator < Item = Span > { let item = tcx . hir_expect_item (def_id) ; if let hir :: ItemKind :: Impl (impl_) = item . kind { let trait_args = impl_ . of_trait . into_iter () . flat_map (| of_trait | of_trait . trait_ref . path . segments . last () . unwrap () . args () . args) . map (| arg | arg . span ()) ; let dummy_spans_for_default_args = impl_ . of_trait . into_iter () . flat_map (| of_trait | iter :: repeat (of_trait . trait_ref . path . span)) ; iter :: once (impl_ . self_ty . span) . chain (trait_args) . chain (dummy_spans_for_default_args) } else { bug ! ("unexpected item for impl {def_id:?}: {item:?}") } }
    };
}

impl_spans!()