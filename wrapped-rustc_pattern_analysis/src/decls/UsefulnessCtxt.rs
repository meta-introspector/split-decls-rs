macro_rules! deps {
    () => {
        BranchPatUsefulness!();
        PatCx!();
        PatId!();
    };
}

macro_rules! UsefulnessCtxt {
    () => {
        deps!();
        # [doc = " Context that provides information for usefulness checking."] struct UsefulnessCtxt < 'a , 'p , Cx : PatCx > { # [doc = " The context for type information."] tycx : & 'a Cx , # [doc = " Track information about the usefulness of branch patterns (see definition of \"branch"] # [doc = " pattern\" at [`BranchPatUsefulness`])."] branch_usefulness : FxHashMap < PatId , BranchPatUsefulness < 'p , Cx > > , complexity_limit : usize , complexity_level : usize , }
    };
}

UsefulnessCtxt!();