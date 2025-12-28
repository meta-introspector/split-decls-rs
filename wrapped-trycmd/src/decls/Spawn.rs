macro_rules! deps {
    () => {
        SpawnStatus!();
    };
}

macro_rules! Spawn {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] struct Spawn { exit : Option < std :: process :: ExitStatus > , status : SpawnStatus , }
    };
}

Spawn!()