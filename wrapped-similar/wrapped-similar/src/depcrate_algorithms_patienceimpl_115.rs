// Generated macro for impl_115 (impl)
macro_rules! Depcrate_algorithms_patienceimpl_115 {
() => {
// Module: crate::algorithms::patience
// Provides: {"impl_115"}
// Dependencies: {}
impl < 'old , 'new , 'd , Old , New , D > DiffHook for Patience < 'old , 'new , 'd , Old , New , D > where D : DiffHook + 'd , Old : Index < usize > + ? Sized + 'old , New : Index < usize > + ? Sized + 'new , New :: Output : PartialEq < Old :: Output > , { type Error = D :: Error ; fn equal (& mut self , old : usize , new : usize , len : usize) -> Result < () , D :: Error > { for (old , new) in (old .. old + len) . zip (new .. new + len) { let a0 = self . old_current ; let b0 = self . new_current ; while self . old_current < self . old_indexes [old] . original_index () && self . new_current < self . new_indexes [new] . original_index () && self . new [self . new_current] == self . old [self . old_current] { self . old_current += 1 ; self . new_current += 1 ; } if self . old_current > a0 { self . d . equal (a0 , b0 , self . old_current - a0) ? ; } let mut no_finish_d = NoFinishHook :: new (& mut self . d) ; myers :: diff_deadline (& mut no_finish_d , self . old , self . old_current .. self . old_indexes [old] . original_index () , self . new , self . new_current .. self . new_indexes [new] . original_index () , self . deadline ,) ? ; self . old_current = self . old_indexes [old] . original_index () ; self . new_current = self . new_indexes [new] . original_index () ; } Ok (()) } fn finish (& mut self) -> Result < () , D :: Error > { myers :: diff_deadline (self . d , self . old , self . old_current .. self . old_end , self . new , self . new_current .. self . new_end , self . deadline ,) } }
};
}
