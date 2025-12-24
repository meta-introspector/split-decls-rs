use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn check_mod_privacy(tcx: TyCtxt<'_>, module_def_id: LocalModDefId) {
    let mut visitor = NamePrivacyVisitor {
        tcx,
        maybe_typeck_results: None,
    };
    tcx.hir_visit_item_likes_in_module(module_def_id, &mut visitor);
    let span = tcx.def_span(module_def_id);
    let mut visitor = TypePrivacyVisitor {
        tcx,
        module_def_id,
        maybe_typeck_results: None,
        span,
    };
    let module = tcx.hir_module_items(module_def_id);
    for def_id in module.definitions() {
        let _ = rustc_ty_utils::sig_types::walk_types(tcx, def_id, &mut visitor);
        if let Some(body_id) = tcx.hir_maybe_body_owned_by(def_id) {
            visitor.visit_nested_body(body_id.id());
        }
        if let DefKind::Impl { of_trait: true } = tcx.def_kind(def_id) {
            let trait_ref = tcx.impl_trait_ref(def_id);
            let trait_ref = trait_ref.instantiate_identity();
            visitor.span = tcx
                .hir_expect_item(def_id)
                .expect_impl()
                .of_trait
                .unwrap()
                .trait_ref
                .path
                .span;
            let _ = visitor
                .visit_def_id(
                    trait_ref.def_id,
                    "trait",
                    &trait_ref.print_only_trait_path(),
                );
        }
    }
}
