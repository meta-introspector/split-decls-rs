macro_rules! deps {
    () => {
        Edition!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl FromStr for Edition { type Err = () ; fn from_str (s : & str) -> Result < Self , () > { match s { "2015" => Ok (Edition :: Edition2015) , "2018" => Ok (Edition :: Edition2018) , "2021" => Ok (Edition :: Edition2021) , "2024" => Ok (Edition :: Edition2024) , "future" => Ok (Edition :: EditionFuture) , _ => Err (()) , } } }
    };
}

impl_34!();