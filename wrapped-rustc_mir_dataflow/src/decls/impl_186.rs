macro_rules! deps {
    () => {
        MovePath!();
        MovePathLinearIter!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < 'tcx > MovePath < 'tcx > { # [doc = " Returns an iterator over the parents of `self`."] pub fn parents < 'a > (& self , move_paths : & 'a IndexSlice < MovePathIndex , MovePath < 'tcx > > ,) -> impl 'a + Iterator < Item = (MovePathIndex , & 'a MovePath < 'tcx >) > { let first = self . parent . map (| mpi | (mpi , & move_paths [mpi])) ; MovePathLinearIter { next : first , fetch_next : move | _ , parent : & MovePath < '_ > | { parent . parent . map (| mpi | (mpi , & move_paths [mpi])) } , } } # [doc = " Returns an iterator over the immediate children of `self`."] pub fn children < 'a > (& self , move_paths : & 'a IndexSlice < MovePathIndex , MovePath < 'tcx > > ,) -> impl 'a + Iterator < Item = (MovePathIndex , & 'a MovePath < 'tcx >) > { let first = self . first_child . map (| mpi | (mpi , & move_paths [mpi])) ; MovePathLinearIter { next : first , fetch_next : move | _ , child : & MovePath < '_ > | { child . next_sibling . map (| mpi | (mpi , & move_paths [mpi])) } , } } # [doc = " Finds the closest descendant of `self` for which `f` returns `true` using a breadth-first"] # [doc = " search."] # [doc = ""] # [doc = " `f` will **not** be called on `self`."] pub fn find_descendant (& self , move_paths : & IndexSlice < MovePathIndex , MovePath < '_ > > , f : impl Fn (MovePathIndex) -> bool ,) -> Option < MovePathIndex > { let mut todo = if let Some (child) = self . first_child { vec ! [child] } else { return None ; } ; while let Some (mpi) = todo . pop () { if f (mpi) { return Some (mpi) ; } let move_path = & move_paths [mpi] ; if let Some (child) = move_path . first_child { todo . push (child) ; } if let Some (sibling) = move_path . next_sibling { todo . push (sibling) ; } } None } }
    };
}

impl_186!();