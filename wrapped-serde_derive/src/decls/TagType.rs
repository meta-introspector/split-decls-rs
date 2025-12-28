macro_rules! TagType {
    () => {
        # [doc = " Styles of representing an enum."] pub enum TagType { # [doc = " The default."] # [doc = ""] # [doc = " ```json"] # [doc = " {\"variant1\": {\"key1\": \"value1\", \"key2\": \"value2\"}}"] # [doc = " ```"] External , # [doc = " `#[serde(tag = \"type\")]`"] # [doc = ""] # [doc = " ```json"] # [doc = " {\"type\": \"variant1\", \"key1\": \"value1\", \"key2\": \"value2\"}"] # [doc = " ```"] Internal { tag : String } , # [doc = " `#[serde(tag = \"t\", content = \"c\")]`"] # [doc = ""] # [doc = " ```json"] # [doc = " {\"t\": \"variant1\", \"c\": {\"key1\": \"value1\", \"key2\": \"value2\"}}"] # [doc = " ```"] Adjacent { tag : String , content : String } , # [doc = " `#[serde(untagged)]`"] # [doc = ""] # [doc = " ```json"] # [doc = " {\"key1\": \"value1\", \"key2\": \"value2\"}"] # [doc = " ```"] None , }
    };
}

TagType!();