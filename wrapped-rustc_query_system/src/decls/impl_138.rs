macro_rules! deps {
    () => {
        QueryStackFrame!();
        QueryState!();
        QueryMap!();
        QueryResult!();
        QueryJobInfo!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < K , I > QueryState < K , I > where K : Eq + Hash + Copy + Debug , { pub fn all_inactive (& self) -> bool { self . active . lock_shards () . all (| shard | shard . is_empty ()) } pub fn try_collect_active_jobs < Qcx : Copy > (& self , qcx : Qcx , make_query : fn (Qcx , K) -> QueryStackFrame < I > , jobs : & mut QueryMap < I > ,) -> Option < () > { let mut active = Vec :: new () ; for shard in self . active . try_lock_shards () { for (k , v) in shard ? . iter () { if let QueryResult :: Started (ref job) = * v { active . push ((* k , (* job) . clone ())) ; } } } for (key , job) in active { let query = make_query (qcx , key) ; jobs . insert (job . id , QueryJobInfo { query , job }) ; } Some (()) } }
    };
}

impl_138!()