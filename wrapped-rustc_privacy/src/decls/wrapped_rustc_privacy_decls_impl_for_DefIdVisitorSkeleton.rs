use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'tcx, V> DefIdVisitorSkeleton<'_, 'tcx, V>
where
    V: DefIdVisitor<'tcx> + ?Sized,
{
    fn visit_trait(&mut self, trait_ref: TraitRef<'tcx>) -> V::Result {
        let TraitRef { def_id, args, .. } = trait_ref;
        try_visit!(
            self.def_id_visitor.visit_def_id(def_id, "trait", & trait_ref
            .print_only_trait_path())
        );
        if V::SHALLOW { V::Result::output() } else { args.visit_with(self) }
    }
    fn visit_projection_term(&mut self, projection: ty::AliasTerm<'tcx>) -> V::Result {
        let tcx = self.def_id_visitor.tcx();
        let (trait_ref, assoc_args) = projection.trait_ref_and_own_args(tcx);
        try_visit!(self.visit_trait(trait_ref));
        if V::SHALLOW {
            V::Result::output()
        } else {
            V::Result::from_branch(
                assoc_args.iter().try_for_each(|arg| arg.visit_with(self).branch()),
            )
        }
    }
    fn visit_clause(&mut self, clause: ty::Clause<'tcx>) -> V::Result {
        match clause.kind().skip_binder() {
            ty::ClauseKind::Trait(ty::TraitPredicate { trait_ref, polarity: _ }) => {
                self.visit_trait(trait_ref)
            }
            ty::ClauseKind::HostEffect(pred) => {
                try_visit!(self.visit_trait(pred.trait_ref));
                pred.constness.visit_with(self)
            }
            ty::ClauseKind::Projection(
                ty::ProjectionPredicate { projection_term: projection_ty, term },
            ) => {
                try_visit!(term.visit_with(self));
                self.visit_projection_term(projection_ty)
            }
            ty::ClauseKind::TypeOutlives(ty::OutlivesPredicate(ty, _region)) => {
                ty.visit_with(self)
            }
            ty::ClauseKind::RegionOutlives(..) => V::Result::output(),
            ty::ClauseKind::ConstArgHasType(ct, ty) => {
                try_visit!(ct.visit_with(self));
                ty.visit_with(self)
            }
            ty::ClauseKind::ConstEvaluatable(ct) => ct.visit_with(self),
            ty::ClauseKind::WellFormed(term) => term.visit_with(self),
            ty::ClauseKind::UnstableFeature(_) => V::Result::output(),
        }
    }
    fn visit_clauses(&mut self, clauses: &[(ty::Clause<'tcx>, Span)]) -> V::Result {
        for &(clause, _) in clauses {
            try_visit!(self.visit_clause(clause));
        }
        V::Result::output()
    }
}
