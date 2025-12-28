macro_rules! deps {
    () => {
        Error!();
        ProcessResult!();
        ObligationForest!();
        TestOutcome!();
    };
}

macro_rules! orphan {
    () => {
        deps!();
        # [test] fn orphan () { let mut forest = ObligationForest :: new () ; forest . register_obligation ("A") ; forest . register_obligation ("B") ; forest . register_obligation ("C1") ; forest . register_obligation ("C2") ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "A" => ProcessResult :: Changed (thin_vec ! ["D" , "E"]) , "B" => ProcessResult :: Unchanged , "C1" => ProcessResult :: Changed (thin_vec ! []) , "C2" => ProcessResult :: Changed (thin_vec ! []) , "D" | "E" => ProcessResult :: Unchanged , _ => unreachable ! () , } , | _ | { } ,)) ; let mut ok = ok ; ok . sort () ; assert_eq ! (ok , vec ! ["C1" , "C2"]) ; assert_eq ! (err . len () , 0) ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "D" | "E" => ProcessResult :: Unchanged , "B" => ProcessResult :: Changed (thin_vec ! ["D"]) , _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (ok . len () , 0) ; assert_eq ! (err . len () , 0) ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "D" => ProcessResult :: Unchanged , "E" => ProcessResult :: Error ("E is for error") , _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (ok . len () , 0) ; assert_eq ! (err , vec ! [super :: Error { error : "E is for error" , backtrace : vec ! ["E" , "A"] }]) ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "D" => ProcessResult :: Error ("D is dead") , _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (ok . len () , 0) ; assert_eq ! (err , vec ! [super :: Error { error : "D is dead" , backtrace : vec ! ["D"] }]) ; let errors = forest . to_errors (()) ; assert_eq ! (errors . len () , 0) ; }
    };
}

orphan!()