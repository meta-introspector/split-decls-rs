macro_rules! deps {
    () => {
        PatStack!();
        PatCx!();
        BranchPatUsefulness!();
    };
}

macro_rules! MatrixRow {
    () => {
        deps!();
        # [doc = " A row of the matrix."] # [derive (Clone)] struct MatrixRow < 'p , Cx : PatCx > { pats : PatStack < 'p , Cx > , # [doc = " Whether the original arm had a guard. This is inherited when specializing."] is_under_guard : bool , # [doc = " When we specialize, we remember which row of the original matrix produced a given row of the"] # [doc = " specialized matrix. When we unspecialize, we use this to propagate usefulness back up the"] # [doc = " callstack. On creation, this stores the index of the original match arm."] parent_row : usize , # [doc = " False when the matrix is just built. This is set to `true` by"] # [doc = " [`compute_exhaustiveness_and_usefulness`] if the arm is found to be useful."] # [doc = " This is reset to `false` when specializing."] useful : bool , # [doc = " Tracks some rows above this one that have an intersection with this one, i.e. such that"] # [doc = " there is a value that matches both rows."] # [doc = " Because of relevancy we may miss some intersections. The intersections we do find are"] # [doc = " correct. In other words, this is an underapproximation of the real set of intersections."] # [doc = ""] # [doc = " For example:"] # [doc = " ```rust,ignore(illustrative)"] # [doc = " match ... {"] # [doc = "     (true, _, _) => {} // `intersects_at_least = []`"] # [doc = "     (_, true, 0..=10) => {} // `intersects_at_least = []`"] # [doc = "     (_, true, 5..15) => {} // `intersects_at_least = [1]`"] # [doc = " }"] # [doc = " ```"] # [doc = " Here the `(true, true)` case is irrelevant. Since we skip it, we will not detect that row 0"] # [doc = " intersects rows 1 and 2."] intersects_at_least : DenseBitSet < usize > , # [doc = " Whether the head pattern is a branch (see definition of \"branch pattern\" at"] # [doc = " [`BranchPatUsefulness`])"] head_is_branch : bool , }
    };
}

MatrixRow!();