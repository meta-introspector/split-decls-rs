macro_rules! DropStyle {
    () => {
        # [doc = " Describes how/if a value should be dropped."] # [derive (Debug)] pub (crate) enum DropStyle { # [doc = " The value is already dead at the drop location, no drop will be executed."] Dead , # [doc = " The value is known to always be initialized at the drop location, drop will always be"] # [doc = " executed."] Static , # [doc = " Whether the value needs to be dropped depends on its drop flag."] Conditional , # [doc = " An \"open\" drop is one where only the fields of a value are dropped."] # [doc = ""] # [doc = " For example, this happens when moving out of a struct field: The rest of the struct will be"] # [doc = " dropped in such an \"open\" drop. It is also used to generate drop glue for the individual"] # [doc = " components of a value, for example for dropping array elements."] Open , }
    };
}

DropStyle!();