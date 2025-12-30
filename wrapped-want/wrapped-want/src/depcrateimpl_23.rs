// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl Taker { # [doc = " Signal to the `Giver` that the want is canceled."] # [doc = ""] # [doc = " This is useful to tell that the channel is closed if you cannot"] # [doc = " drop the value yet."] # [inline] pub fn cancel (& mut self) { self . signal (State :: Closed) } # [doc = " Signal to the `Giver` that a value is wanted."] # [inline] pub fn want (& mut self) { debug_assert ! (self . inner . state . load (SeqCst) != State :: Closed . into () , "want called after cancel") ; self . signal (State :: Want) } # [inline] fn signal (& mut self , state : State) { let old_state = self . inner . state . swap (state . into () , SeqCst) . into () ; match old_state { State :: Idle | State :: Want | State :: Closed => () , State :: Give => { loop { if let Some (mut locked) = self . inner . task . try_lock_explicit (SeqCst , SeqCst) { if let Some (task) = locked . take () { drop (locked) ; task . wake () ; } return ; } else { } } } , } } }
};
}
