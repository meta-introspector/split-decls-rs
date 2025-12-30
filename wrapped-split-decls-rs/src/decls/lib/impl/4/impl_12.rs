use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl syn :: parse :: Parse for ConditionalWrapperInput { fn parse (input : syn :: parse :: ParseStream) -> syn :: Result < Self > { input . parse :: < syn :: Ident > () ? ; input . parse :: < syn :: Token ! [=] > () ? ; let feature = input . parse () ? ; input . parse :: < syn :: Token ! [,] > () ? ; input . parse :: < syn :: Ident > () ? ; input . parse :: < syn :: Token ! [=] > () ? ; let original = input . parse () ? ; input . parse :: < syn :: Token ! [,] > () ? ; input . parse :: < syn :: Ident > () ? ; input . parse :: < syn :: Token ! [=] > () ? ; let wrapper = input . parse () ? ; Ok (ConditionalWrapperInput { feature , original , wrapper , }) } }
}