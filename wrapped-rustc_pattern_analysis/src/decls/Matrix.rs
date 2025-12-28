macro_rules! deps {
    () => {
        MatrixRow!();
        PlaceInfo!();
        PatCx!();
    };
}

macro_rules! Matrix {
    () => {
        deps!();
        # [doc = " A 2D matrix. Represents a list of pattern-tuples under investigation."] # [doc = ""] # [doc = " Invariant: each row must have the same length, and each column must have the same type."] # [doc = ""] # [doc = " Invariant: the first column must not contain or-patterns. This is handled by"] # [doc = " [`Matrix::push`]."] # [doc = ""] # [doc = " In fact each column corresponds to a place inside the scrutinee of the match. E.g. after"] # [doc = " specializing `(,)` and `Some` on a pattern of type `(Option<u32>, bool)`, the first column of"] # [doc = " the matrix will correspond to `scrutinee.0.Some.0` and the second column to `scrutinee.1`."] # [derive (Clone)] struct Matrix < 'p , Cx : PatCx > { # [doc = " Vector of rows. The rows must form a rectangular 2D array. Moreover, all the patterns of"] # [doc = " each column must have the same type. Each column corresponds to a place within the"] # [doc = " scrutinee."] rows : Vec < MatrixRow < 'p , Cx > > , # [doc = " Track info about each place. Each place corresponds to a column in `rows`, and their types"] # [doc = " must match."] place_info : SmallVec < [PlaceInfo < Cx > ; 2] > , # [doc = " Track whether the virtual wildcard row used to compute exhaustiveness is relevant. See top"] # [doc = " of the file for details on relevancy."] wildcard_row_is_relevant : bool , }
    };
}

Matrix!()