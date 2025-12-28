macro_rules! deps {
    () => {
        PatCx!();
        PatOrWild!();
    };
}

macro_rules! PatStack {
    () => {
        deps!();
        # [doc = " Represents a pattern-tuple under investigation."] struct PatStack < 'p , Cx : PatCx > { pats : SmallVec < [PatOrWild < 'p , Cx > ; 2] > , # [doc = " Sometimes we know that as far as this row is concerned, the current case is already handled"] # [doc = " by a different, more general, case. When the case is irrelevant for all rows this allows us"] # [doc = " to skip a case entirely. This is purely an optimization. See at the top for details."] relevant : bool , }
    };
}

PatStack!()