use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < L , R > Either < & mut L , & mut R > { # [doc = " Maps an `Either<&mut L, &mut R>` to an `Either<L, R>` by cloning the contents of"] # [doc = " either branch."] pub fn cloned (self) -> Either < L , R > where L : Clone , R : Clone , { map_either ! (self , inner => inner . clone ()) } # [doc = " Maps an `Either<&mut L, &mut R>` to an `Either<L, R>` by copying the contents of"] # [doc = " either branch."] pub fn copied (self) -> Either < L , R > where L : Copy , R : Copy , { map_either ! (self , inner => * inner) } }