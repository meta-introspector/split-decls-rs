macro_rules! deps {
    () => {
        TransferFunction!();
        Analysis!();
        MaybeTransitiveLiveLocals!();
        Direction!();
        Backward!();
        YieldResumeEffect!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < 'a , 'tcx > Analysis < 'tcx > for MaybeTransitiveLiveLocals < 'a > { type Domain = DenseBitSet < Local > ; type Direction = Backward ; const NAME : & 'static str = "transitive liveness" ; fn bottom_value (& self , body : & mir :: Body < 'tcx >) -> Self :: Domain { DenseBitSet :: new_empty (body . local_decls . len ()) } fn initialize_start_block (& self , _ : & mir :: Body < 'tcx > , _ : & mut Self :: Domain) { } fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , statement : & mir :: Statement < 'tcx > , location : Location ,) { let destination = match & statement . kind { StatementKind :: Assign (assign) => assign . 1 . is_safe_to_remove () . then_some (assign . 0) , StatementKind :: SetDiscriminant { place , .. } | StatementKind :: Deinit (place) => { Some (* * place) } StatementKind :: FakeRead (_) | StatementKind :: StorageLive (_) | StatementKind :: StorageDead (_) | StatementKind :: Retag (..) | StatementKind :: AscribeUserType (..) | StatementKind :: PlaceMention (..) | StatementKind :: Coverage (..) | StatementKind :: Intrinsic (..) | StatementKind :: ConstEvalCounter | StatementKind :: BackwardIncompatibleDropHint { .. } | StatementKind :: Nop => None , } ; if let Some (destination) = destination { if ! destination . is_indirect () && ! state . contains (destination . local) && ! self . always_live . contains (destination . local) { return ; } } TransferFunction (state) . visit_statement (statement , location) ; } fn apply_primary_terminator_effect < 'mir > (& mut self , state : & mut Self :: Domain , terminator : & 'mir mir :: Terminator < 'tcx > , location : Location ,) -> TerminatorEdges < 'mir , 'tcx > { TransferFunction (state) . visit_terminator (terminator , location) ; terminator . edges () } fn apply_call_return_effect (& mut self , state : & mut Self :: Domain , _block : mir :: BasicBlock , return_places : CallReturnPlaces < '_ , 'tcx > ,) { if let CallReturnPlaces :: Yield (resume_place) = return_places { YieldResumeEffect (state) . visit_place (& resume_place , PlaceContext :: MutatingUse (MutatingUseContext :: Yield) , Location :: START ,) } else { return_places . for_each (| place | { if let Some (local) = place . as_local () { state . remove (local) ; } }) ; } } }
    };
}

impl_162!()