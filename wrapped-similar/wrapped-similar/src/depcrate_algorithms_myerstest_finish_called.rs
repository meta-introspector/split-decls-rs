// Generated macro for test_finish_called (function)
macro_rules! Depcrate_algorithms_myerstest_finish_called {
() => {
// Module: crate::algorithms::myers
// Provides: {"test_finish_called"}
// Dependencies: {}
# [test] fn test_finish_called () { struct HasRunFinish (bool) ; impl DiffHook for HasRunFinish { type Error = () ; fn finish (& mut self) -> Result < () , Self :: Error > { self . 0 = true ; Ok (()) } } let mut d = HasRunFinish (false) ; let slice = & [1 , 2] ; let slice2 = & [1 , 2 , 3] ; diff (& mut d , slice , 0 .. slice . len () , slice2 , 0 .. slice2 . len ()) . unwrap () ; assert ! (d . 0) ; let mut d = HasRunFinish (false) ; let slice = & [1 , 2] ; diff (& mut d , slice , 0 .. slice . len () , slice , 0 .. slice . len ()) . unwrap () ; assert ! (d . 0) ; let mut d = HasRunFinish (false) ; let slice : & [u8] = & [] ; diff (& mut d , slice , 0 .. slice . len () , slice , 0 .. slice . len ()) . unwrap () ; assert ! (d . 0) ; }
};
}
