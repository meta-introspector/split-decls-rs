// Generated macro for action_kind_strategy (function)
macro_rules! Depcrate_tests_propertiesaction_kind_strategy {
() => {
// Module: crate::tests::properties
// Provides: {"action_kind_strategy"}
// Dependencies: {}
fn action_kind_strategy () -> impl Strategy < Value = ActionKind > { prop_oneof ! [1 => Just (ActionKind :: Insert) , 1 => Just (ActionKind :: VacantEntry) , 1 => prop :: num :: usize :: ANY . prop_map (ActionKind :: RemoveRandom) , 1 => prop :: num :: usize :: ANY . prop_map (ActionKind :: RemoveExistent) , 1 => prop :: num :: usize :: ANY . prop_map (ActionKind :: TakeRandom) , 1 => prop :: num :: usize :: ANY . prop_map (ActionKind :: TakeExistent) , 5 => prop :: num :: usize :: ANY . prop_map (ActionKind :: GetRandom) , 5 => prop :: num :: usize :: ANY . prop_map (ActionKind :: GetExistent) ,] }
};
}
