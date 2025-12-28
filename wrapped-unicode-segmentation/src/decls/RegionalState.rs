macro_rules! RegionalState {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq , Debug)] enum RegionalState { Half , Full , Unknown , }
    };
}

RegionalState!();