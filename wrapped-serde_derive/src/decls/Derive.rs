macro_rules! Derive {
    () => {
        # [derive (Copy , Clone)] pub enum Derive { Serialize , Deserialize , }
    };
}

Derive!()