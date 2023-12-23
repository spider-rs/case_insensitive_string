#[cfg(feature = "serde")]
pub extern crate serde;

#[cfg(feature = "compact")]
pub extern crate compact_str;

pub mod features;

/// case-insensitive string handling
#[cfg(not(feature = "compact"))]
#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct CaseInsensitiveString(String);

/// case-insensitive string handling
#[cfg(feature = "compact")]
#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct CaseInsensitiveString(compact_str::CompactString);

impl PartialEq for CaseInsensitiveString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for CaseInsensitiveString {}

impl std::hash::Hash for CaseInsensitiveString {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_ascii_lowercase().hash(state)
    }
}

impl From<&str> for CaseInsensitiveString {
    #[inline]
    fn from(s: &str) -> Self {
        CaseInsensitiveString { 0: s.into() }
    }
}

#[cfg(feature = "compact")]
impl From<compact_str::CompactString> for CaseInsensitiveString {
    #[inline]
    fn from(s: compact_str::CompactString) -> Self {
        CaseInsensitiveString { 0: s.into() }
    }
}

impl From<String> for CaseInsensitiveString {
    fn from(s: String) -> Self {
        CaseInsensitiveString { 0: s.into() }
    }
}

impl From<&[u8]> for CaseInsensitiveString {
    fn from(s: &[u8]) -> Self {
        CaseInsensitiveString {
            0: String::from_utf8_lossy(s).into(),
        }
    }
}

impl AsRef<str> for CaseInsensitiveString {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl CaseInsensitiveString {
    /// Creates a `CaseInsensitiveString` slice from any byte slice.
    ///
    ///
    /// This is a cost-free conversion.
    ///
    /// # Example
    ///
    /// You can create `CaseInsensitiveString`'s from byte arrays, byte slices or string slices:
    ///
    /// ```
    /// use case_insensitive_string::CaseInsensitiveString;
    ///
    /// let a = CaseInsensitiveString::new(b"abc");
    /// let b = CaseInsensitiveString::new("abc");
    ///
    /// assert_eq!(a, b);
    /// ```
    #[inline]
    pub fn new<'a, B: ?Sized + AsRef<[u8]>>(bytes: &'a B) -> CaseInsensitiveString {
        CaseInsensitiveString::from(bytes.as_ref())
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0.as_bytes()
    }

    #[inline]
    #[cfg(not(feature = "compact"))]
    pub fn inner(&self) -> &String {
        &self.0
    }

    #[cfg(feature = "compact")]
    #[inline]
    pub fn inner(&self) -> &compact_str::CompactString {
        &self.0
    }
}
