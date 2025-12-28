macro_rules! update_env {
    () => {
        fn update_env < K , V > (key : K , value : Option < V >) where K : AsRef < OsStr > , V : AsRef < OsStr > , { match value { Some (v) => env :: set_var (key , v) , None => env :: remove_var (key) , } }
    };
}

update_env!();