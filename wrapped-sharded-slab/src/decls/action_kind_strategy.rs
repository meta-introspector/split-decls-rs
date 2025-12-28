macro_rules! deps {
    () => {
        ActionKind!();
        VacantEntry!();
    };
}

macro_rules! action_kind_strategy {
    () => {
        deps!();
        fn action_kind_strategy () -> impl Strategy < Value = ActionKind > { prop_oneof ! [1 => Just (ActionKind :: Insert) , 1 => Just (ActionKind :: VacantEntry) , 1 => prop :: num :: usize :: ANY . prop_map (ActionKind :: RemoveRandom) , 1 => prop :: num :: usize :: ANY . prop_map (ActionKind :: RemoveExistent) , 1 => prop :: num :: usize :: ANY . prop_map (ActionKind :: TakeRandom) , 1 => prop :: num :: usize :: ANY . prop_map (ActionKind :: TakeExistent) , 5 => prop :: num :: usize :: ANY . prop_map (ActionKind :: GetRandom) , 5 => prop :: num :: usize :: ANY . prop_map (ActionKind :: GetExistent) ,] }
    };
}

action_kind_strategy!();