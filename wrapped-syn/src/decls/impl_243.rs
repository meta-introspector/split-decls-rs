macro_rules! impl_243 {
    () => {
        impl PartialEq for Member { fn eq (& self , other : & Self) -> bool { match (self , other) { (Member :: Named (this) , Member :: Named (other)) => this == other , (Member :: Unnamed (this) , Member :: Unnamed (other)) => this == other , _ => false , } } }
    };
}

impl_243!()