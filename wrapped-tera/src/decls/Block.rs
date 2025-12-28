macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! Block {
    () => {
        deps!();
        # [doc = " A block definition"] # [derive (Clone , Debug , PartialEq)] pub struct Block { # [doc = " The block name"] pub name : String , # [doc = " The block content"] pub body : Vec < Node > , }
    };
}

Block!();