macro_rules! deps {
    () => {
        Review!();
    };
}

macro_rules! impl_498 {
    () => {
        deps!();
        impl Review { # [allow (dead_code)] pub fn new () -> Review { Review { title : "My review" . to_owned () , paragraphs : vec ! ["A" . to_owned () , "B" . to_owned () , "C" . to_owned ()] , } } }
    };
}

impl_498!();