macro_rules! deps {
    () => {
        Lint!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < 'a , 'tcx > Visitor < 'tcx > for Lint < 'a , 'tcx > { fn visit_local (& mut self , local : Local , context : PlaceContext , location : Location) { if context . is_use () { self . maybe_storage_dead . seek_after_primary_effect (location) ; if self . maybe_storage_dead . get () . contains (local) { self . fail (location , format ! ("use of local {local:?}, which has no storage here")) ; } } } fn visit_statement (& mut self , statement : & Statement < 'tcx > , location : Location) { match & statement . kind { StatementKind :: Assign (box (dest , rvalue)) => { if let Rvalue :: Use (Operand :: Copy (src) | Operand :: Move (src)) = rvalue { if dest == src { self . fail (location , "encountered `Assign` statement with overlapping memory" ,) ; } } } StatementKind :: StorageLive (local) => { self . maybe_storage_live . seek_before_primary_effect (location) ; if self . maybe_storage_live . get () . contains (* local) { self . fail (location , format ! ("StorageLive({local:?}) which already has storage here") ,) ; } } _ => { } } self . super_statement (statement , location) ; } fn visit_terminator (& mut self , terminator : & Terminator < 'tcx > , location : Location) { match & terminator . kind { TerminatorKind :: Return => { if self . is_fn_like { self . maybe_storage_live . seek_after_primary_effect (location) ; for local in self . maybe_storage_live . get () . iter () { if ! self . always_live_locals . contains (local) { self . fail (location , format ! ("local {local:?} still has storage when returning from function") ,) ; } } } } TerminatorKind :: Call { args , destination , .. } => { self . places . clear () ; self . places . insert (destination . as_ref ()) ; let mut has_duplicates = false ; for arg in args { if let Operand :: Move (place) = & arg . node { has_duplicates |= ! self . places . insert (place . as_ref ()) ; } } if has_duplicates { self . fail (location , format ! ("encountered overlapping memory in `Move` arguments to `Call` terminator: {:?}" , terminator . kind ,) ,) ; } } _ => { } } self . super_terminator (terminator , location) ; } }
    };
}

impl_81!();