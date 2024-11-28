#[cfg(feature = "serde")]
pub extern crate serde;

#[cfg(feature = "compact")]
pub extern crate compact_str;

pub mod features;

#[cfg(feature = "compact")]
use compact_str::CompactString;
/// case-insensitive string handling
#[cfg(not(feature = "compact"))]
#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct CaseInsensitiveString(String);

/// case-insensitive string handling
#[cfg(feature = "compact")]
#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct CaseInsensitiveString(CompactString);

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

    #[cfg(not(feature = "compact"))]
    #[inline]
    pub fn inner(&self) -> &String {
        &self.0
    }

    #[cfg(feature = "compact")]
    #[inline]
    pub fn inner(&self) -> &CompactString {
        &self.0
    }

    /// Appends the given [`char`] to the end of this [`CaseInsensitiveString`].
    ///
    /// # Examples
    /// ```
    /// # use case_insensitive_string::CaseInsensitiveString;
    /// let mut s = CaseInsensitiveString::new("foo");
    ///
    /// s.push('b');
    /// s.push('a');
    /// s.push('r');
    ///
    /// assert_eq!(CaseInsensitiveString::from("foobar"), s);
    /// ```
    pub fn push(&mut self, ch: char) {
        self.push_str(ch.encode_utf8(&mut [0; 4]));
    }

    /// Appends a given string slice onto the end of this [`CaseInsensitiveString`]
    ///
    /// # Examples
    /// ```
    /// # use case_insensitive_string::CaseInsensitiveString;
    /// let mut s = CaseInsensitiveString::new("abc");
    ///
    /// s.push_str("123");
    ///
    /// assert_eq!(CaseInsensitiveString::new("abc123"), s);
    /// ```
    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.0.push_str(s)
    }

    /// Convert the [`CaseInsensitiveString`] into a [`String`].
    /// ```
    pub fn into_string(self) -> String {
        self.0.into()
    }

    /// Removes a [`char`] from this [`CaseInsensitiveString`] at a byte position and returns it.
    ///
    /// This is an *O*(*n*) operation, as it requires copying every element in the
    /// buffer.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is larger than or equal to the [`CaseInsensitiveString`]'s length,
    /// or if it does not lie on a [`char`] boundary.
    ///
    /// # Examples
    ///
    /// ### Basic usage:
    ///
    /// ```
    /// # use case_insensitive_string::CaseInsensitiveString;
    /// let mut c = CaseInsensitiveString::from("hello world");
    ///
    /// assert_eq!(c.remove(0), 'h');
    /// assert_eq!(c, "ello world".into());
    ///
    /// assert_eq!(c.remove(5), 'w');
    /// assert_eq!(c, "ello orld".into());
    /// ```
    ///
    /// ### Past total length:
    ///
    /// ```should_panic
    /// # use case_insensitive_string::CaseInsensitiveString;
    /// let mut c = CaseInsensitiveString::from("hello there!");
    /// c.remove(100);
    /// ```
    ///
    /// ### Not on char boundary:
    ///
    /// ```should_panic
    /// # use case_insensitive_string::CaseInsensitiveString;
    /// let mut c = CaseInsensitiveString::from("🦄");
    /// c.remove(1);
    /// ```
    #[inline]
    pub fn remove(&mut self, idx: usize) -> char {
        self.0.remove(idx)
    }

    /// Returns the length of the [`CaseInsensitiveString`] in `bytes`, not [`char`]s or graphemes.
    ///
    /// When using `UTF-8` encoding (which all strings in Rust do) a single character will be 1 to 4
    /// bytes long, therefore the return value of this method might not be what a human considers
    /// the length of the string.
    ///
    /// # Examples
    /// ```
    /// # use case_insensitive_string::CaseInsensitiveString;
    /// let ascii = CaseInsensitiveString::new("hello world");
    /// assert_eq!(ascii.len(), 11);
    ///
    /// let emoji = CaseInsensitiveString::new("👱");
    /// assert_eq!(emoji.len(), 4);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the [`CaseInsensitiveString`] has a length of 0, `false` otherwise
    ///
    /// # Examples
    /// ```
    /// # use case_insensitive_string::CaseInsensitiveString;
    /// let mut msg = CaseInsensitiveString::new("");
    /// assert!(msg.is_empty());
    ///
    /// // add some characters
    /// msg.push_str("hello reader!");
    /// assert!(!msg.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
impl From<CompactString> for CaseInsensitiveString {
    #[inline]
    fn from(s: CompactString) -> Self {
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

impl From<CaseInsensitiveString> for String {
    #[inline]
    fn from(s: CaseInsensitiveString) -> Self {
        s.into_string()
    }
}

impl From<&CaseInsensitiveString> for String {
    #[inline]
    fn from(s: &CaseInsensitiveString) -> Self {
        s.0.to_string()
    }
}

impl From<&CaseInsensitiveString> for CaseInsensitiveString {
    #[inline]
    fn from(s: &CaseInsensitiveString) -> Self {
        s.clone()
    }
}

impl AsRef<str> for CaseInsensitiveString {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for CaseInsensitiveString {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for CaseInsensitiveString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0.as_str()
    }
}

impl std::ops::DerefMut for CaseInsensitiveString {
    #[inline]
    fn deref_mut(&mut self) -> &mut str {
        self.0.as_mut_str()
    }
}

impl std::borrow::Borrow<str> for CaseInsensitiveString {
    #[inline]
    fn borrow(&self) -> &str {
        &self.0.as_str()
    }
}

impl PartialEq for CaseInsensitiveString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

#[cfg(feature = "compact")]
impl PartialEq<CaseInsensitiveString> for &CompactString {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(&other.as_ref())
    }
}

impl PartialEq<CaseInsensitiveString> for String {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(&other.as_ref())
    }
}

impl<'a> PartialEq<&'a CaseInsensitiveString> for String {
    fn eq(&self, other: &&CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(&other.as_ref())
    }
}

impl PartialEq<CaseInsensitiveString> for &String {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(&other.as_ref())
    }
}

impl PartialEq<CaseInsensitiveString> for str {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(&other.as_ref())
    }
}

impl<'a> PartialEq<&'a CaseInsensitiveString> for str {
    fn eq(&self, other: &&CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(&other.as_ref())
    }
}

impl PartialEq<CaseInsensitiveString> for &str {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(&other.as_ref())
    }
}

impl PartialEq<CaseInsensitiveString> for &&str {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(&other.as_ref())
    }
}

impl<'a> PartialEq<CaseInsensitiveString> for std::borrow::Cow<'a, str> {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(&other.as_ref())
    }
}

impl<'a> PartialEq<CaseInsensitiveString> for &std::borrow::Cow<'a, str> {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(&other.as_ref())
    }
}

impl PartialEq<String> for &CaseInsensitiveString {
    fn eq(&self, other: &String) -> bool {
        self.eq_ignore_ascii_case(&other.as_ref())
    }
}

impl<'a> PartialEq<std::borrow::Cow<'a, str>> for &CaseInsensitiveString {
    fn eq(&self, other: &std::borrow::Cow<'a, str>) -> bool {
        self.eq_ignore_ascii_case(&other.as_ref())
    }
}
