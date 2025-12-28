macro_rules! set_global_default {
    () => {
        # [doc = " Sets this subscriber as the global default for the duration of the entire program."] # [doc = " Will be used as a fallback if no thread-local subscriber has been set in a thread (using `with_default`.)"] # [doc = ""] # [doc = " Can only be set once; subsequent attempts to set the global default will fail."] # [doc = " Returns whether the initialization was successful."] # [doc = ""] # [doc = " Note: Libraries should *NOT* call `set_global_default()`! That will cause conflicts when"] # [doc = " executables try to set them later."] # [doc = ""] # [doc = " [`Subscriber`]: super::subscriber::Subscriber"] # [doc = " [`Event`]: super::event::Event"] pub fn set_global_default < S > (subscriber : S) -> Result < () , SetGlobalDefaultError > where S : Subscriber + Send + Sync + 'static , { crate :: dispatcher :: set_global_default (crate :: Dispatch :: new (subscriber)) }
    };
}

set_global_default!()