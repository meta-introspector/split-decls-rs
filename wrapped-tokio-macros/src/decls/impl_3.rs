macro_rules! deps {
    () => {
        RuntimeFlavor!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl RuntimeFlavor { fn from_str (s : & str) -> Result < RuntimeFlavor , String > { match s { "current_thread" => Ok (RuntimeFlavor :: CurrentThread) , "multi_thread" => Ok (RuntimeFlavor :: Threaded) , "local" => Ok (RuntimeFlavor :: Local) , "single_thread" => Err ("The single threaded runtime flavor is called `current_thread`." . to_string ()) , "basic_scheduler" => Err ("The `basic_scheduler` runtime flavor has been renamed to `current_thread`." . to_string ()) , "threaded_scheduler" => Err ("The `threaded_scheduler` runtime flavor has been renamed to `multi_thread`." . to_string ()) , _ => Err (format ! ("No such runtime flavor `{s}`. The runtime flavors are `current_thread`, `local`, and `multi_thread`.")) , } } }
    };
}

impl_3!()