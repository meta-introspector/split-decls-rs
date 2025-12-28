macro_rules! deps {
    () => {
        QueryState!();
    };
}

macro_rules! JobOwner {
    () => {
        deps!();
        # [doc = " A type representing the responsibility to execute the job in the `job` field."] # [doc = " This will poison the relevant query if dropped."] struct JobOwner < 'tcx , K , I > where K : Eq + Hash + Copy , { state : & 'tcx QueryState < K , I > , key : K , }
    };
}

JobOwner!()