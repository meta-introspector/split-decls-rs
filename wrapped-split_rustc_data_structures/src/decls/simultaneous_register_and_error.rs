macro_rules! deps {
    () => {
        TestOutcome!();
        ProcessResult!();
        ObligationForest!();
        Error!();
    };
}

macro_rules! simultaneous_register_and_error {
    () => {
        deps!();
        # [test] fn simultaneous_register_and_error () { let mut forest = ObligationForest :: new () ; forest . register_obligation ("A") ; forest . register_obligation ("B") ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "A" => ProcessResult :: Error ("An error") , "B" => ProcessResult :: Changed (thin_vec ! ["A"]) , _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (ok . len () , 0) ; assert_eq ! (err , vec ! [super :: Error { error : "An error" , backtrace : vec ! ["A"] }]) ; let mut forest = ObligationForest :: new () ; forest . register_obligation ("B") ; forest . register_obligation ("A") ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "A" => ProcessResult :: Error ("An error") , "B" => ProcessResult :: Changed (thin_vec ! ["A"]) , _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (ok . len () , 0) ; assert_eq ! (err , vec ! [super :: Error { error : "An error" , backtrace : vec ! ["A"] }]) ; }
    };
}

simultaneous_register_and_error!();