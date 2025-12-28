macro_rules! duration_to_secs_str {
    () => {
        pub fn duration_to_secs_str (dur : std :: time :: Duration) -> String { format ! ("{:.3}" , dur . as_secs_f64 ()) }
    };
}

duration_to_secs_str!()