macro_rules! SpawnStatus {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum SpawnStatus { Ok , Skipped , Failure (crate :: Error) , Expected (String) , }
    };
}

SpawnStatus!();