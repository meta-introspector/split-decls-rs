macro_rules! deps {
    () => {
        Error!();
        Result!();
        SortNumbers!();
        SortStrategy!();
        SortBools!();
        SortArrays!();
        SortStrings!();
    };
}

macro_rules! get_sort_strategy_for_type {
    () => {
        deps!();
        pub fn get_sort_strategy_for_type (ty : & Value) -> Result < Box < dyn SortStrategy > > { use crate :: Value :: * ; match * ty { Null => Err (Error :: msg ("Null is not a sortable value")) , Bool (_) => Ok (Box :: < SortBools > :: default ()) , Number (_) => Ok (Box :: < SortNumbers > :: default ()) , String (_) => Ok (Box :: < SortStrings > :: default ()) , Array (_) => Ok (Box :: < SortArrays > :: default ()) , Object (_) => Err (Error :: msg ("Object is not a sortable value")) , } }
    };
}

get_sort_strategy_for_type!()