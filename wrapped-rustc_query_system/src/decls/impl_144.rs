macro_rules! deps {
    () => {
        JobOwner!();
        QueryResult!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < 'tcx , K , I > Drop for JobOwner < 'tcx , K , I > where K : Eq + Hash + Copy , { # [inline (never)] # [cold] fn drop (& mut self) { let state = self . state ; let job = { let key_hash = sharded :: make_hash (& self . key) ; let mut shard = state . active . lock_shard_by_hash (key_hash) ; match shard . find_entry (key_hash , equivalent_key (& self . key)) { Err (_) => panic ! () , Ok (occupied) => { let ((key , value) , vacant) = occupied . remove () ; vacant . insert ((key , QueryResult :: Poisoned)) ; value . expect_job () } } } ; job . signal_complete () ; } }
    };
}

impl_144!()