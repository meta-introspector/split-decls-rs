macro_rules! macro_71 {
    () => {
        ast_enum ! { # [doc = " Distinguishes between attributes that decorate an item and attributes"] # [doc = " that are contained within an item."] # [doc = ""] # [doc = " # Outer attributes"] # [doc = ""] # [doc = " - `#[repr(transparent)]`"] # [doc = " - `/// # Example`"] # [doc = " - `/** Please file an issue */`"] # [doc = ""] # [doc = " # Inner attributes"] # [doc = ""] # [doc = " - `#![feature(proc_macro)]`"] # [doc = " - `//! # Example`"] # [doc = " - `/*! Please file an issue */`"] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum AttrStyle { Outer , Inner (Token ! [!]) , } }
    };
}

macro_71!()