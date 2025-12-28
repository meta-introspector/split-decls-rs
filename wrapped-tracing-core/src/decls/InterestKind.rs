macro_rules! InterestKind {
    () => {
        # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd)] enum InterestKind { Never = 0 , Sometimes = 1 , Always = 2 , }
    };
}

InterestKind!();