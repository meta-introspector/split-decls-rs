macro_rules! deps {
    () => {
        PathKind!();
        StackEntry!();
        Cx!();
        Stack!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < X : Cx > Stack < X > { pub (super) fn is_empty (& self) -> bool { self . entries . is_empty () } pub (super) fn len (& self) -> usize { self . entries . len () } pub (super) fn last (& self) -> Option < & StackEntry < X > > { self . entries . raw . last () } pub (super) fn last_mut (& mut self) -> Option < & mut StackEntry < X > > { self . entries . raw . last_mut () } pub (super) fn last_mut_with_index (& mut self) -> Option < (StackDepth , & mut StackEntry < X >) > { self . entries . last_index () . map (| idx | (idx , & mut self . entries [idx])) } pub (super) fn next_index (& self) -> StackDepth { self . entries . next_index () } pub (super) fn push (& mut self , entry : StackEntry < X >) -> StackDepth { if cfg ! (debug_assertions) && self . entries . iter () . any (| e | e . input == entry . input) { panic ! ("pushing duplicate entry on stack: {entry:?} {:?}" , self . entries) ; } self . entries . push (entry) } pub (super) fn pop (& mut self) -> StackEntry < X > { self . entries . pop () . unwrap () } pub (super) fn cycle_step_kinds (& self , head : StackDepth) -> impl Iterator < Item = PathKind > { self . entries . raw [head . index () + 1 ..] . iter () . map (| entry | entry . step_kind_from_parent) } pub (super) fn iter (& self) -> impl Iterator < Item = & StackEntry < X > > { self . entries . iter () } pub (super) fn find (& self , input : X :: Input) -> Option < StackDepth > { self . entries . iter_enumerated () . find (| (_ , e) | e . input == input) . map (| (idx , _) | idx) } }
    };
}

impl_131!()