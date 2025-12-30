// Generated macro for Patience (struct)
macro_rules! Depcrate_algorithms_patiencePatience {
() => {
// Module: crate::algorithms::patience
// Provides: {"Patience"}
// Dependencies: {}
struct Patience < 'old , 'new , 'd , Old : ? Sized , New : ? Sized , D > { d : & 'd mut D , old : & 'old Old , old_current : usize , old_end : usize , old_indexes : & 'old [UniqueItem < 'old , Old >] , new : & 'new New , new_current : usize , new_end : usize , new_indexes : & 'new [UniqueItem < 'new , New >] , deadline : Option < Instant > , }
};
}
