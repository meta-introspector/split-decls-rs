macro_rules! FieldConfig {
    () => {
        # [derive (Clone)] enum FieldConfig { Default , Skip , With (syn :: Path) , }
    };
}

FieldConfig!();