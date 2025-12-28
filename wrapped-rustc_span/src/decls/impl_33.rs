macro_rules! deps {
    () => {
        Edition!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Edition { pub fn lint_name (self) -> & 'static str { match self { Edition :: Edition2015 => "rust_2015_compatibility" , Edition :: Edition2018 => "rust_2018_compatibility" , Edition :: Edition2021 => "rust_2021_compatibility" , Edition :: Edition2024 => "rust_2024_compatibility" , Edition :: EditionFuture => "edition_future_compatibility" , } } pub fn is_stable (self) -> bool { match self { Edition :: Edition2015 => true , Edition :: Edition2018 => true , Edition :: Edition2021 => true , Edition :: Edition2024 => true , Edition :: EditionFuture => false , } } # [doc = " Is this edition 2015?"] pub fn is_rust_2015 (self) -> bool { self == Edition :: Edition2015 } # [doc = " Are we allowed to use features from the Rust 2018 edition?"] pub fn at_least_rust_2018 (self) -> bool { self >= Edition :: Edition2018 } # [doc = " Are we allowed to use features from the Rust 2021 edition?"] pub fn at_least_rust_2021 (self) -> bool { self >= Edition :: Edition2021 } # [doc = " Are we allowed to use features from the Rust 2024 edition?"] pub fn at_least_rust_2024 (self) -> bool { self >= Edition :: Edition2024 } # [doc = " Are we allowed to use features from the future edition?"] pub fn at_least_edition_future (self) -> bool { self >= Edition :: EditionFuture } }
    };
}

impl_33!();