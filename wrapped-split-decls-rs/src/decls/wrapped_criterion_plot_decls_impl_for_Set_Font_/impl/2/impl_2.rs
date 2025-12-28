use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Set < Font > for Figure { # [doc = " Changes the font"] fn set (& mut self , font : Font) -> & mut Figure { self . font = Some (font . 0) ; self } }