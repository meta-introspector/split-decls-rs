// Generated macro for impl_122 (impl)
macro_rules! Depcrate_androidimpl_122 {
() => {
// Module: crate::android
// Provides: {"impl_122"}
// Dependencies: {}
impl < 'a , 'env > LocalContext < 'a , 'env > { # [doc = " Load a class from the application class loader"] # [doc = ""] # [doc = " This should be used instead of `JNIEnv::find_class` to ensure all classes"] # [doc = " in the application can be found."] fn load_class (& mut self , name : & str) -> Result < JClass < 'env > , Error > { let name = self . env . new_string (name) ? ; let class = self . env . call_method (& self . loader , "loadClass" , "(Ljava/lang/String;)Ljava/lang/Class;" , & [JValue :: from (& name)] ,) ? ; Ok (JObject :: try_from (class) ? . into ()) } # [doc = " Borrow the `applicationContext` from the Android application"] # [doc = " <https://developer.android.com/reference/android/app/Application>"] pub (super) fn application_context (& self) -> & JObject < '_ > { & self . context } }
};
}
