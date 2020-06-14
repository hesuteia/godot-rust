use crate::private::get_api;
use crate::sys;
use crate::RefCounted;

use std::cmp::Ordering;
use std::ffi::CStr;
use std::fmt;
use std::mem::forget;
use std::ops::Range;
use std::slice;
use std::str;

/// Godot's reference-counted string type.
pub struct GodotString(pub(crate) sys::godot_string);

macro_rules! impl_methods {
    // Methods that return a GodotString:
    (
        $(pub fn $method:ident(&self) -> Self : $gd_method:ident;)*
    ) => {
        $(
            #[inline]
            pub fn $method(&self) -> Self {
                unsafe {
                    GodotString((get_api().$gd_method)(&self.0))
                }
            }
        )*
    };

    // Methods that return a basic type:
    (
        $(pub fn $method:ident(&self) -> $Type:ty : $gd_method:ident;)*
    ) => {
        $(
            #[inline]
            pub fn $method(&self) -> $Type {
                unsafe { (get_api().$gd_method)(&self.0) }
            }
        )*
    };
}

impl GodotString {
    #[inline]
    pub fn new() -> Self {
        GodotString::default()
    }

    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S>(s: S) -> Self
    where
        S: AsRef<str>,
    {
        unsafe {
            let api = get_api();
            let val = s.as_ref();
            let godot_s =
                (api.godot_string_chars_to_utf8_with_len)(val.as_ptr() as *const _, val.len() as _);

            GodotString(godot_s)
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { (get_api().godot_string_length)(&self.0) as usize }
    }

    impl_methods!(
        pub fn is_empty(&self) -> bool : godot_string_empty;
        pub fn is_numeric(&self) -> bool : godot_string_is_numeric;
        pub fn is_valid_float(&self) -> bool : godot_string_is_valid_float;
        pub fn is_valid_html_color(&self) -> bool : godot_string_is_valid_html_color;
        pub fn is_valid_identifier(&self) -> bool : godot_string_is_valid_identifier;
        pub fn is_valid_integer(&self) -> bool : godot_string_is_valid_integer;
        pub fn is_valid_ip_address(&self) -> bool : godot_string_is_valid_ip_address;
        pub fn is_resource_file(&self) -> bool : godot_string_is_resource_file;
        pub fn is_absolute_path(&self) -> bool : godot_string_is_abs_path;
        pub fn is_relative_path(&self) -> bool : godot_string_is_rel_path;
        pub fn to_f32(&self) -> f32 : godot_string_to_float;
        pub fn to_f64(&self) -> f64 : godot_string_to_double;
        pub fn to_i32(&self) -> i32 : godot_string_to_int;
        pub fn u32_hash(&self) -> u32 : godot_string_hash;
        pub fn u64_hash(&self) -> u64 : godot_string_hash64;
        pub fn hex_to_int(&self) -> i32 : godot_string_hex_to_int;
        pub fn hex_to_int_without_prefix(&self) -> i32 : godot_string_hex_to_int_without_prefix;
    );

    impl_methods!(
        pub fn camelcase_to_underscore(&self) -> Self : godot_string_camelcase_to_underscore;
        pub fn camelcase_to_underscore_lowercased(&self) -> Self : godot_string_camelcase_to_underscore_lowercased;
        pub fn capitalize(&self) -> Self : godot_string_capitalize;
        pub fn to_lowercase(&self) -> Self : godot_string_to_lower;
        pub fn to_uppercase(&self) -> Self : godot_string_to_upper;
        pub fn get_file(&self) -> Self : godot_string_get_file;
        pub fn get_base_dir(&self) -> Self : godot_string_get_base_dir;
        pub fn simplify_path(&self) -> Self : godot_string_simplify_path;
        pub fn sha256_text(&self) -> Self : godot_string_sha256_text;
        pub fn md5_text(&self) -> Self : godot_string_md5_text;
        pub fn c_escape(&self) -> Self : godot_string_c_escape;
        pub fn c_escape_multiline(&self) -> Self : godot_string_c_escape_multiline;
        pub fn c_unescape(&self) -> Self : godot_string_c_unescape;
        pub fn http_escape(&self) -> Self : godot_string_http_escape;
        pub fn http_unescape(&self) -> Self: godot_string_http_unescape;
        pub fn json_escape(&self) -> Self : godot_string_json_escape;
        pub fn xml_escape(&self) -> Self : godot_string_xml_escape;
        pub fn xml_escape_with_quotes(&self) -> Self : godot_string_xml_escape_with_quotes;
        pub fn xml_unescape(&self) -> Self: godot_string_xml_unescape;
        pub fn percent_decode(&self) -> Self : godot_string_percent_decode;
        pub fn percent_encode(&self) -> Self : godot_string_percent_encode;
    );

    #[inline]
    pub fn is_valid_hex_number(&self, with_prefix: bool) -> bool {
        unsafe { (get_api().godot_string_is_valid_hex_number)(&self.0, with_prefix) }
    }

    #[inline]
    pub fn begins_with(&self, s: &GodotString) -> bool {
        unsafe { (get_api().godot_string_begins_with)(&self.0, &s.0) }
    }

    #[inline]
    pub fn ends_with(&self, s: &GodotString) -> bool {
        unsafe { (get_api().godot_string_ends_with)(&self.0, &s.0) }
    }

    #[inline]
    pub fn begins_with_c_str(&self, s: &CStr) -> bool {
        unsafe { (get_api().godot_string_begins_with_char_array)(&self.0, s.as_ptr()) }
    }

    #[inline]
    pub fn sub_string(&self, range: Range<usize>) -> Self {
        unsafe {
            let count = range.end - range.start;
            GodotString((get_api().godot_string_substr)(
                &self.0,
                range.start as i32,
                count as i32,
            ))
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn to_utf8(&self) -> Utf8String {
        unsafe { Utf8String((get_api().godot_string_utf8)(&self.0)) }
    }

    #[inline]
    pub fn find(&self, what: &GodotString) -> i32 {
        unsafe { (get_api().godot_string_find)(&self.0, what.0) }
    }

    #[inline]
    pub fn find_from(&self, what: &GodotString, from: i32) -> i32 {
        unsafe { (get_api().godot_string_find_from)(&self.0, what.0, from) }
    }

    #[inline]
    pub fn find_last(&self, what: &GodotString) -> i32 {
        unsafe { (get_api().godot_string_find_last)(&self.0, what.0) }
    }

    /// Returns the internal ffi representation of the string and consumes
    /// the rust object without running the destructor.
    ///
    /// This should be only used when certain that the receiving side is
    /// responsible for running the destructor for the object, otherwise
    /// it is leaked.
    #[inline]
    pub fn forget(self) -> sys::godot_string {
        let v = self.0;
        forget(self);
        v
    }

    /// Returns a copy of the internal ffi representation of the string.
    ///
    /// The string remains owned by the rust wrapper and the receiver of
    /// the ffi representation should not run its destructor.
    #[inline]
    pub fn to_sys(&self) -> sys::godot_string {
        self.0
    }

    #[doc(hidden)]
    #[inline]
    pub fn sys(&self) -> *const sys::godot_string {
        &self.0
    }

    #[doc(hidden)]
    #[inline]
    pub fn sys_mut(&mut self) -> *mut sys::godot_string {
        &mut self.0
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_sys(sys: sys::godot_string) -> Self {
        GodotString(sys)
    }
    // TODO: many missing methods.
}

impl Clone for GodotString {
    #[inline]
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl_basic_traits_as_sys!(
    for GodotString as godot_string {
        Drop => godot_string_destroy;
        Eq => godot_string_operator_equal;
        Default => godot_string_new;
        RefCounted => godot_string_new_copy;
    }
);

impl fmt::Display for GodotString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let utf8 = self.to_utf8();
        f.write_str(utf8.as_str())
    }
}

impl fmt::Debug for GodotString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.to_string().fmt(f)
    }
}

impl std::hash::Hash for GodotString {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.u64_hash());
    }
}

// TODO: Is it useful to expose this type?
// Could just make it an internal detail of how to convert to a rust string.
#[doc(hidden)]
pub struct Utf8String(pub(crate) sys::godot_char_string);

impl Utf8String {
    #[inline]
    pub fn len(&self) -> i32 {
        unsafe { (get_api().godot_char_string_length)(&self.0) }
    }

    /// Returns `true` if `self` has a length of zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let data = (get_api().godot_char_string_get_data)(&self.0) as _;
            slice::from_raw_parts(data, self.len() as usize)
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    #[doc(hidden)]
    #[inline]
    pub fn sys(&self) -> *const sys::godot_char_string {
        &self.0
    }

    #[doc(hidden)]
    #[inline]
    pub fn sys_mut(&mut self) -> *mut sys::godot_char_string {
        &mut self.0
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_sys(sys: sys::godot_char_string) -> Self {
        Self(sys)
    }
}

impl ToString for Utf8String {
    #[inline]
    fn to_string(&self) -> String {
        String::from(self.as_str())
    }
}

impl_basic_traits_as_sys!(
    for Utf8String as godot_char_string {
        Drop => godot_char_string_destroy;
    }
);

impl fmt::Debug for Utf8String {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.to_string().fmt(f)
    }
}

pub struct StringName(pub(crate) sys::godot_string_name);

impl StringName {
    #[inline]
    pub fn from_str<S>(s: S)
    where
        S: AsRef<str>,
    {
        let gd_string = GodotString::from_str(s);
        StringName::from_godot_string(&gd_string);
    }

    #[inline]
    pub fn from_c_str(s: &CStr) -> Self {
        unsafe {
            let mut result = sys::godot_string_name::default();
            (get_api().godot_string_name_new_data)(&mut result, s.as_ptr());
            StringName(result)
        }
    }

    #[inline]
    pub fn from_godot_string(s: &GodotString) -> Self {
        unsafe {
            let mut result = sys::godot_string_name::default();
            (get_api().godot_string_name_new)(&mut result, &s.0);
            StringName(result)
        }
    }

    #[inline]
    pub fn get_hash(&self) -> u32 {
        unsafe { (get_api().godot_string_name_get_hash)(&self.0) }
    }

    #[inline]
    pub fn get_name(&self) -> GodotString {
        unsafe { GodotString((get_api().godot_string_name_get_name)(&self.0)) }
    }

    #[inline]
    pub fn operator_less(&self, s: &StringName) -> bool {
        unsafe { (get_api().godot_string_name_operator_less)(&self.0, &s.0) }
    }

    #[doc(hidden)]
    #[inline]
    pub fn sys(&self) -> *const sys::godot_string_name {
        &self.0
    }

    #[doc(hidden)]
    #[inline]
    pub fn sys_mut(&mut self) -> *mut sys::godot_string_name {
        &mut self.0
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_sys(sys: sys::godot_string_name) -> Self {
        Self(sys)
    }
}

impl_basic_traits_as_sys! {
    for StringName as godot_string_name {
        Drop => godot_string_name_destroy;
        Eq => godot_string_name_operator_equal;
    }
}

impl fmt::Debug for StringName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.get_name().to_string().fmt(f)
    }
}

impl PartialOrd for StringName {
    #[inline]
    fn partial_cmp(&self, other: &StringName) -> Option<Ordering> {
        unsafe {
            let native = (get_api().godot_string_name_operator_less)(&self.0, &other.0);

            if native {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        }
    }
}

impl<S> From<S> for GodotString
where
    S: AsRef<str>,
{
    #[inline]
    fn from(s: S) -> GodotString {
        GodotString::from_str(s)
    }
}

godot_test!(test_string {
    use crate::{GodotString, Variant, VariantType};

    let foo: GodotString = "foo".into();
    assert_eq!(foo.len(), 3);
    assert_eq!(foo.to_string(), String::from("foo"));

    let foo2 = foo.new_ref();
    assert!(foo == foo2);

    let variant = Variant::from_godot_string(&foo);
    assert!(variant.get_type() == VariantType::GodotString);

    let variant2: Variant = "foo".into();
    assert!(variant == variant2);

    if let Some(foo_variant) = variant.try_to_godot_string() {
        assert!(foo_variant == foo);
    } else {
        panic!("variant should be a GodotString");
    }

    assert_eq!(foo.to_utf8().as_str(), "foo");
});
