// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use icu_properties::{script, Script};

    use crate::errors::ffi::ICU4XError;
    use diplomat_runtime::DiplomatResult;

    #[diplomat::opaque]
    /// An ICU4X ScriptWithExtensions map object, capable of holding a map of codepoints to scriptextensions values
    #[diplomat::rust_link(icu::properties::script::ScriptWithExtensions, Struct)]
    pub struct ICU4XScriptWithExtensions(pub script::ScriptWithExtensions);

    #[diplomat::opaque]
    /// A slightly faster ICU4XScriptWithExtensions object
    #[diplomat::rust_link(icu::properties::script::ScriptWithExtensionsBorrowed, Struct)]
    pub struct ICU4XScriptWithExtensionsBorrowed<'a>(pub script::ScriptWithExtensionsBorrowed<'a>);
    #[diplomat::opaque]
    /// An object that represents the Script_Extensions property for a single character
    #[diplomat::rust_link(icu::properties::script::ScriptExtensionsSet, Struct)]
    pub struct ICU4XScriptExtensionsSet<'a>(pub script::ScriptExtensionsSet<'a>);

    impl ICU4XScriptWithExtensions {
        #[diplomat::rust_link(icu::properties::script::load_script_with_extensions_unstable, Fn)]
        pub fn create(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XScriptWithExtensions>, ICU4XError> {
            script::load_script_with_extensions_unstable(&provider.0)
                .map(|data| Box::new(ICU4XScriptWithExtensions(data)))
                .map_err(Into::into)
                .into()
        }

        /// Get the Script property value for a code point
        #[diplomat::rust_link(
            icu::properties::script::ScriptWithExtensionsBorrowed::get_script_val,
            FnInStruct
        )]
        pub fn get_script_val(&self, code_point: u32) -> u16 {
            self.0.as_borrowed().get_script_val(code_point).0
        }

        /// Check if the Script_Extensions property of the given code point covers the given script
        #[diplomat::rust_link(
            icu::properties::script::ScriptWithExtensionsBorrowed::has_script,
            FnInStruct
        )]
        pub fn has_script(&self, code_point: u32, script: u16) -> bool {
            self.0.as_borrowed().has_script(code_point, Script(script))
        }

        /// Borrow this object for a slightly faster variant with more operations
        #[diplomat::rust_link(
            icu::properties::script::ScriptWithExtensions::as_borrowed,
            FnInStruct
        )]
        pub fn as_borrowed<'a>(&'a self) -> Box<ICU4XScriptWithExtensionsBorrowed<'a>> {
            Box::new(ICU4XScriptWithExtensionsBorrowed(self.0.as_borrowed()))
        }
    }

    impl<'a> ICU4XScriptWithExtensionsBorrowed<'a> {
        /// Get the Script property value for a code point
        #[diplomat::rust_link(
            icu::properties::script::ScriptWithExtensionsBorrowed::get_script_val,
            FnInStruct
        )]
        pub fn get_script_val(&self, code_point: u32) -> u16 {
            self.0.get_script_val(code_point).0
        }
        /// Get the Script property value for a code point
        #[diplomat::rust_link(
            icu::properties::script::ScriptWithExtensionsBorrowed::get_script_extensions_val,
            FnInStruct
        )]
        pub fn get_script_extensions_val(
            &self,
            code_point: u32,
        ) -> Box<ICU4XScriptExtensionsSet<'a>> {
            Box::new(ICU4XScriptExtensionsSet(
                self.0.get_script_extensions_val(code_point),
            ))
        }
        /// Check if the Script_Extensions property of the given code point covers the given script
        #[diplomat::rust_link(
            icu::properties::script::ScriptWithExtensionsBorrowed::has_script,
            FnInStruct
        )]
        pub fn has_script(&self, code_point: u32, script: u16) -> bool {
            self.0.has_script(code_point, Script(script))
        }
    }
    impl<'a> ICU4XScriptExtensionsSet<'a> {
        /// Check if the Script_Extensions property of the given code point covers the given script
        #[diplomat::rust_link(icu::properties::script::ScriptExtensionsSet::contains, FnInStruct)]
        pub fn contains(&self, script: u16) -> bool {
            self.0.contains(&Script(script))
        }

        /// Get the number of scripts contained in here
        #[diplomat::rust_link(icu::properties::script::ScriptExtensionsSet::iter, FnInStruct)]
        pub fn count(&self) -> usize {
            self.0.array_len()
        }

        /// Get script at index, returning an error if out of bounds
        #[diplomat::rust_link(icu::properties::script::ScriptExtensionsSet::iter, FnInStruct)]
        pub fn script_at(&self, index: usize) -> DiplomatResult<u16, ()> {
            self.0.array_get(index).map(|x| x.0).ok_or(()).into()
        }
    }
}
