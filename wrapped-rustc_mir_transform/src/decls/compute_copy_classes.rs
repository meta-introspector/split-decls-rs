macro_rules! deps {
    () => {
        SsaLocals!();
    };
}

macro_rules! compute_copy_classes {
    () => {
        deps!();
        # [instrument (level = "trace" , skip (ssa , body))] fn compute_copy_classes (ssa : & mut SsaLocals , body : & Body < '_ >) { let mut direct_uses = std :: mem :: take (& mut ssa . direct_uses) ; let mut copies = IndexVec :: from_fn_n (| l | l , body . local_decls . len ()) ; for (local , rvalue , _) in ssa . assignments (body) { let (Rvalue :: Use (Operand :: Copy (place) | Operand :: Move (place)) | Rvalue :: CopyForDeref (place)) = rvalue else { continue ; } ; let Some (rhs) = place . as_local () else { continue } ; let local_ty = body . local_decls () [local] . ty ; let rhs_ty = body . local_decls () [rhs] . ty ; if local_ty != rhs_ty { trace ! ("skipped `{local:?} = {rhs:?}` due to subtyping: {local_ty} != {rhs_ty}") ; continue ; } if ! ssa . is_ssa (rhs) { continue ; } let head = copies [rhs] ; if ssa . borrowed_locals () . contains (local) { continue ; } if local == RETURN_PLACE { if body . local_kind (head) != LocalKind :: Temp { continue ; } for h in copies . iter_mut () { if * h == head { * h = RETURN_PLACE ; } } } else { copies [local] = head ; } direct_uses [rhs] -= 1 ; } debug ! (? copies) ; debug ! (? direct_uses) ; # [cfg (debug_assertions)] for & head in copies . iter () { assert_eq ! (copies [head] , head) ; } debug_assert_eq ! (copies [RETURN_PLACE] , RETURN_PLACE) ; ssa . direct_uses = direct_uses ; ssa . copy_classes = copies ; }
    };
}

compute_copy_classes!();