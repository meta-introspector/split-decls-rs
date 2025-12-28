macro_rules! StreamStatus {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] enum StreamStatus { Ok , Failure (crate :: Error) , Expected (crate :: Data) , }
    };
}

StreamStatus!();