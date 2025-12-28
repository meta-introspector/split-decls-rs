macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! ProcessResult {
    () => {
        deps!();
        # [doc = " The result type used by `process_obligation`."] # [repr (C)] # [derive (Debug)] pub enum ProcessResult < O , E > { Unchanged , Changed (ThinVec < O >) , Error (E) , }
    };
}

ProcessResult!()