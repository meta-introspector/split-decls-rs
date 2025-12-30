// Generated macro for HirIdValidator (struct)
macro_rules! Depcrate_hir_id_validatorHirIdValidator {
() => {
// Module: crate::hir_id_validator
// Provides: {"HirIdValidator"}
// Dependencies: {}
struct HirIdValidator < 'a , 'hir > { tcx : TyCtxt < 'hir > , owner : Option < hir :: OwnerId > , hir_ids_seen : GrowableBitSet < ItemLocalId > , errors : & 'a Lock < Vec < String > > , }
};
}
