use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > syn :: parse :: Parse for Parenthesized < T > where T : syn :: parse :: Parse , { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let content ; syn :: parenthesized ! (content in input) ; content . parse :: < T > () . map (Parenthesized) } }