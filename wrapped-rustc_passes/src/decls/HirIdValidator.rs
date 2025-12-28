macro_rules! HirIdValidator {
    () => {
        struct HirIdValidator < 'a , 'hir > { tcx : TyCtxt < 'hir > , owner : Option < hir :: OwnerId > , hir_ids_seen : GrowableBitSet < ItemLocalId > , errors : & 'a Lock < Vec < String > > , }
    };
}

HirIdValidator!();