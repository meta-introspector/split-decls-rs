use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'tcx> EmbargoVisitor<'tcx> {
    fn check_def_id(&mut self, owner_id: OwnerId) {
        let item_ev = self.get(owner_id.def_id);
        match self.tcx.def_kind(owner_id) {
            DefKind::Use | DefKind::ExternCrate | DefKind::GlobalAsm => {}
            DefKind::Mod => {}
            DefKind::Macro { .. } => {
                if let Some(item_ev) = item_ev {
                    let (_, macro_def, _) =
                        self.tcx.hir_expect_item(owner_id.def_id).expect_macro();
                    self.update_reachability_from_macro(owner_id.def_id, macro_def, item_ev);
                }
            }
            DefKind::ForeignTy
            | DefKind::Const
            | DefKind::Static { .. }
            | DefKind::Fn
            | DefKind::TyAlias => {
                if let Some(item_ev) = item_ev {
                    self.reach(owner_id.def_id, item_ev)
                        .generics()
                        .predicates()
                        .ty();
                }
            }
            DefKind::Trait => {
                if let Some(item_ev) = item_ev {
                    self.reach(owner_id.def_id, item_ev).generics().predicates();
                    for assoc_item in self.tcx.associated_items(owner_id).in_definition_order() {
                        if assoc_item.is_impl_trait_in_trait() {
                            continue;
                        }
                        let def_id = assoc_item.def_id.expect_local();
                        self.update(def_id, item_ev, Level::Reachable);
                        let tcx = self.tcx;
                        let mut reach = self.reach(def_id, item_ev);
                        reach.generics().predicates();
                        if assoc_item.is_type() && !assoc_item.defaultness(tcx).has_value() {
                        } else {
                            reach.ty();
                        }
                    }
                }
            }
            DefKind::TraitAlias => {
                if let Some(item_ev) = item_ev {
                    self.reach(owner_id.def_id, item_ev).generics().predicates();
                }
            }
            DefKind::Impl { of_trait } => {
                let item_ev = EffectiveVisibility::of_impl::<true>(
                    owner_id.def_id,
                    of_trait,
                    self.tcx,
                    &self.effective_visibilities,
                );
                self.update_eff_vis(owner_id.def_id, item_ev, None, Level::Direct);
                {
                    let mut reach = self.reach(owner_id.def_id, item_ev);
                    reach.generics().predicates().ty();
                    if of_trait {
                        reach.trait_ref();
                    }
                }
                for assoc_item in self.tcx.associated_items(owner_id).in_definition_order() {
                    if assoc_item.is_impl_trait_in_trait() {
                        continue;
                    }
                    let def_id = assoc_item.def_id.expect_local();
                    let max_vis = if of_trait {
                        None
                    } else {
                        Some(self.tcx.local_visibility(def_id))
                    };
                    self.update_eff_vis(def_id, item_ev, max_vis, Level::Direct);
                    if let Some(impl_item_ev) = self.get(def_id) {
                        self.reach(def_id, impl_item_ev)
                            .generics()
                            .predicates()
                            .ty();
                    }
                }
            }
            DefKind::Enum => {
                if let Some(item_ev) = item_ev {
                    self.reach(owner_id.def_id, item_ev).generics().predicates();
                }
                let def = self.tcx.adt_def(owner_id);
                for variant in def.variants() {
                    if let Some(item_ev) = item_ev {
                        self.update(variant.def_id.expect_local(), item_ev, Level::Reachable);
                    }
                    if let Some(variant_ev) = self.get(variant.def_id.expect_local()) {
                        if let Some(ctor_def_id) = variant.ctor_def_id() {
                            self.update(ctor_def_id.expect_local(), variant_ev, Level::Reachable);
                        }
                        for field in &variant.fields {
                            let field = field.did.expect_local();
                            self.update(field, variant_ev, Level::Reachable);
                            self.reach(field, variant_ev).ty();
                        }
                        self.reach(owner_id.def_id, variant_ev).ty();
                    }
                    if let Some(ctor_def_id) = variant.ctor_def_id() {
                        if let Some(ctor_ev) = self.get(ctor_def_id.expect_local()) {
                            self.reach(owner_id.def_id, ctor_ev).ty();
                        }
                    }
                }
            }
            DefKind::Struct | DefKind::Union => {
                let def = self.tcx.adt_def(owner_id).non_enum_variant();
                if let Some(item_ev) = item_ev {
                    self.reach(owner_id.def_id, item_ev).generics().predicates();
                    for field in &def.fields {
                        let field = field.did.expect_local();
                        self.update(field, item_ev, Level::Reachable);
                        if let Some(field_ev) = self.get(field) {
                            self.reach(field, field_ev).ty();
                        }
                    }
                }
                if let Some(ctor_def_id) = def.ctor_def_id() {
                    if let Some(item_ev) = item_ev {
                        self.update(ctor_def_id.expect_local(), item_ev, Level::Reachable);
                    }
                    if let Some(ctor_ev) = self.get(ctor_def_id.expect_local()) {
                        self.reach(owner_id.def_id, ctor_ev).ty();
                    }
                }
            }
            DefKind::ForeignMod => {}
            DefKind::Field
            | DefKind::Variant
            | DefKind::AssocFn
            | DefKind::AssocTy
            | DefKind::AssocConst
            | DefKind::TyParam
            | DefKind::AnonConst
            | DefKind::InlineConst
            | DefKind::OpaqueTy
            | DefKind::Closure
            | DefKind::SyntheticCoroutineBody
            | DefKind::ConstParam
            | DefKind::LifetimeParam
            | DefKind::Ctor(..) => bug!("should be checked while checking parent"),
        }
    }
}
