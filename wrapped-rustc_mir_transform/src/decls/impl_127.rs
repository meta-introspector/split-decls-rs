macro_rules! deps {
    () => {
        SsaVisitor!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for SsaVisitor < '_ , 'tcx > { fn visit_local (& mut self , local : Local , ctxt : PlaceContext , loc : Location) { if ctxt . may_observe_address () { self . borrowed_locals . insert (local) ; } match ctxt { PlaceContext :: MutatingUse (MutatingUseContext :: Projection) | PlaceContext :: NonMutatingUse (NonMutatingUseContext :: Projection) => bug ! () , PlaceContext :: NonMutatingUse (NonMutatingUseContext :: RawBorrow) | PlaceContext :: MutatingUse (_) => { self . assignments [local] = Set1 :: Many ; } PlaceContext :: NonMutatingUse (NonMutatingUseContext :: SharedBorrow | NonMutatingUseContext :: FakeBorrow ,) => { self . check_dominates (local , loc) ; self . direct_uses [local] += 1 ; } PlaceContext :: NonMutatingUse (_) => { self . check_dominates (local , loc) ; self . direct_uses [local] += 1 ; } PlaceContext :: NonUse (_) => { } } } fn visit_place (& mut self , place : & Place < 'tcx > , ctxt : PlaceContext , loc : Location) { let location = match ctxt { PlaceContext :: MutatingUse (MutatingUseContext :: Store) => { Some (DefLocation :: Assignment (loc)) } PlaceContext :: MutatingUse (MutatingUseContext :: Call) => { let call = loc . block ; let TerminatorKind :: Call { target , .. } = self . body . basic_blocks [call] . terminator () . kind else { bug ! () } ; Some (DefLocation :: CallReturn { call , target }) } _ => None , } ; if let Some (location) = location && let Some (local) = place . as_local () { self . assignments [local] . insert (location) ; if let Set1 :: One (_) = self . assignments [local] { self . assignment_order . push (local) ; } } else if place . projection . first () == Some (& PlaceElem :: Deref) { if ctxt . is_use () { let new_ctxt = PlaceContext :: NonMutatingUse (NonMutatingUseContext :: Copy) ; self . visit_projection (place . as_ref () , new_ctxt , loc) ; self . check_dominates (place . local , loc) ; } } else { self . visit_projection (place . as_ref () , ctxt , loc) ; self . visit_local (place . local , ctxt , loc) ; } } }
    };
}

impl_127!();