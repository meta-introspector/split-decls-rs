macro_rules! deps {
    () => {
        TestOutcome!();
        ProcessResult!();
        Error!();
        ObligationForest!();
    };
}

macro_rules! diamond {
    () => {
        deps!();
        # [test] fn diamond () { let mut forest = ObligationForest :: new () ; forest . register_obligation ("A") ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "A" => ProcessResult :: Changed (thin_vec ! ["A.1" , "A.2"]) , "A.1" | "A.2" => ProcessResult :: Unchanged , _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (ok . len () , 0) ; assert_eq ! (err . len () , 0) ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "A.1" => ProcessResult :: Changed (thin_vec ! ["D"]) , "A.2" => ProcessResult :: Changed (thin_vec ! ["D"]) , "D" => ProcessResult :: Unchanged , _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (ok . len () , 0) ; assert_eq ! (err . len () , 0) ; let mut d_count = 0 ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "D" => { d_count += 1 ; ProcessResult :: Changed (thin_vec ! []) } _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (d_count , 1) ; let mut ok = ok ; ok . sort () ; assert_eq ! (ok , vec ! ["A" , "A.1" , "A.2" , "D"]) ; assert_eq ! (err . len () , 0) ; let errors = forest . to_errors (()) ; assert_eq ! (errors . len () , 0) ; forest . register_obligation ("A'") ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "A'" => ProcessResult :: Changed (thin_vec ! ["A'.1" , "A'.2"]) , "A'.1" | "A'.2" => ProcessResult :: Unchanged , _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (ok . len () , 0) ; assert_eq ! (err . len () , 0) ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "A'.1" => ProcessResult :: Changed (thin_vec ! ["D'" , "A'"]) , "A'.2" => ProcessResult :: Changed (thin_vec ! ["D'"]) , "D'" | "A'" => ProcessResult :: Unchanged , _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (ok . len () , 0) ; assert_eq ! (err . len () , 0) ; let mut d_count = 0 ; let TestOutcome { completed : ok , errors : err , .. } = forest . process_obligations (& mut C (| obligation | match * obligation { "D'" => { d_count += 1 ; ProcessResult :: Error ("operation failed") } _ => unreachable ! () , } , | _ | { } ,)) ; assert_eq ! (d_count , 1) ; assert_eq ! (ok . len () , 0) ; assert_eq ! (err , vec ! [super :: Error { error : "operation failed" , backtrace : vec ! ["D'" , "A'.1" , "A'"] }]) ; let errors = forest . to_errors (()) ; assert_eq ! (errors . len () , 0) ; }
    };
}

diamond!()