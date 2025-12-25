use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'tcx> TraitChecker<'tcx, TyCtxt<'tcx>, DefId, MockTy<'tcx>> for MockTyCtxtWrapper<'tcx> {
    fn get_trait_def_id(&self, _trait_name: &str) -> Option<DefId> {
        Some(DefId)
    }
    fn type_implements_trait(
        &self,
        _tcx: TyCtxt<'tcx>,
        _adt_ty: MockTy<'tcx>,
        _item_def_id: DefId,
        _trait_def_id: DefId,
    ) -> bool {
        true
    }
}
