macro_rules! LinkSelfContainedComponentsWrapper {
    () => {
        # [derive (serde_derive :: Deserialize , schemars :: JsonSchema)] struct LinkSelfContainedComponentsWrapper { components : Vec < LinkSelfContainedComponents > , }
    };
}

LinkSelfContainedComponentsWrapper!()