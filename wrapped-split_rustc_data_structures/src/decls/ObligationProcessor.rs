macro_rules! deps {
    () => {
        ProcessResult!();
        OutcomeTrait!();
        ForestObligation!();
        Error!();
    };
}

macro_rules! ObligationProcessor {
    () => {
        deps!();
        pub trait ObligationProcessor { type Obligation : ForestObligation ; type Error : Debug ; type OUT : OutcomeTrait < Obligation = Self :: Obligation , Error = Error < Self :: Obligation , Self :: Error > > ; # [doc = " Implementations can provide a fast-path to obligation-processing"] # [doc = " by counting the prefix of the passed iterator for which"] # [doc = " `needs_process_obligation` would return false."] fn skippable_obligations < 'a > (& 'a self , _it : impl Iterator < Item = & 'a Self :: Obligation > ,) -> usize { 0 } fn needs_process_obligation (& self , _obligation : & Self :: Obligation) -> bool ; fn process_obligation (& mut self , obligation : & mut Self :: Obligation ,) -> ProcessResult < Self :: Obligation , Self :: Error > ; # [doc = " As we do the cycle check, we invoke this callback when we"] # [doc = " encounter an actual cycle. `cycle` is an iterator that starts"] # [doc = " at the start of the cycle in the stack and walks **toward the"] # [doc = " top**."] # [doc = ""] # [doc = " In other words, if we had O1 which required O2 which required"] # [doc = " O3 which required O1, we would give an iterator yielding O1,"] # [doc = " O2, O3 (O1 is not yielded twice)."] fn process_backedge < 'c , I > (& mut self , cycle : I , _marker : PhantomData < & 'c Self :: Obligation > ,) -> Result < () , Self :: Error > where I : Clone + Iterator < Item = & 'c Self :: Obligation > ; }
    };
}

ObligationProcessor!();