macro_rules! DataAndPosition {
    () => {
        pub (crate) struct DataAndPosition < T > { pub data : T , # [allow (dead_code)] pub position : u64 , }
    };
}

DataAndPosition!()