macro_rules! Unique {
    () => {
        # [derive (Default)] pub struct Unique < K : Eq + std :: hash :: Hash > { unique : std :: collections :: HashSet < K > , }
    };
}

Unique!()