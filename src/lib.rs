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
    pub fn new<B: ?Sized + AsRef<[u8]>>(bytes: &B) -> CaseInsensitiveString {
        CaseInsensitiveString::from(bytes.as_ref())
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
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
        // Zero-alloc case-folding: feed lowercased bytes directly into the
        // hasher without allocating a temporary string.
        let bytes = self.0.as_bytes();

        // Hash the length first (matches str's Hash contract: length-prefixed).
        bytes.len().hash(state);

        #[cfg(target_arch = "x86_64")]
        {
            hash_case_fold_simd(bytes, state);
        }

        #[cfg(target_arch = "aarch64")]
        {
            hash_case_fold_simd_neon(bytes, state);
        }

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            hash_case_fold_scalar(bytes, state);
        }
    }
}

/// Scalar fallback: fold ASCII uppercase → lowercase one byte at a time.
#[inline(always)]
#[allow(dead_code)]
fn hash_case_fold_scalar<H: std::hash::Hasher>(bytes: &[u8], state: &mut H) {
    for &b in bytes {
        state.write_u8(b.to_ascii_lowercase());
    }
}

/// SSE2 path: SIMD case-fold into a stack buffer, then hash in one shot.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn hash_case_fold_simd<H: std::hash::Hasher>(bytes: &[u8], state: &mut H) {
    use core::arch::x86_64::*;

    let len = bytes.len();

    // Short strings: scalar is cheaper than SIMD setup.
    if len < 16 {
        hash_case_fold_scalar(bytes, state);
        return;
    }

    // Stack buffer for strings up to 512 bytes; heap fallback for longer.
    // The single state.write() at the end is critical for hasher throughput.
    if len <= 512 {
        let mut buf = [0u8; 512];
        unsafe { case_fold_simd_x86(bytes, buf.as_mut_ptr(), len) };
        state.write(&buf[..len]);
    } else {
        let mut buf = vec![0u8; len];
        unsafe { case_fold_simd_x86(bytes, buf.as_mut_ptr(), len) };
        state.write(&buf);
    }
}

/// Raw SSE2 case-fold: lowercase `src[..len]` into `dst`.
///
/// # Safety
/// `dst` must have at least `len` bytes of writable space.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn case_fold_simd_x86(src: &[u8], dst: *mut u8, len: usize) {
    use core::arch::x86_64::*;

    let upper_a = _mm_set1_epi8(b'A' as i8);
    let upper_z = _mm_set1_epi8(b'Z' as i8);
    let lower_bit = _mm_set1_epi8(0x20);
    let mut i = 0;

    while i + 16 <= len {
        let chunk = _mm_loadu_si128(src.as_ptr().add(i) as *const __m128i);
        let ge_a = _mm_cmpgt_epi8(chunk, _mm_sub_epi8(upper_a, _mm_set1_epi8(1)));
        let le_z = _mm_cmpgt_epi8(_mm_add_epi8(upper_z, _mm_set1_epi8(1)), chunk);
        let mask = _mm_and_si128(ge_a, le_z);
        let lowered = _mm_or_si128(chunk, _mm_and_si128(mask, lower_bit));
        _mm_storeu_si128(dst.add(i) as *mut __m128i, lowered);
        i += 16;
    }

    // Scalar tail.
    while i < len {
        *dst.add(i) = src[i].to_ascii_lowercase();
        i += 1;
    }
}

/// NEON path (aarch64): SIMD case-fold into a stack buffer, then hash in one shot.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn hash_case_fold_simd_neon<H: std::hash::Hasher>(bytes: &[u8], state: &mut H) {
    let len = bytes.len();

    if len < 16 {
        hash_case_fold_scalar(bytes, state);
        return;
    }

    if len <= 512 {
        let mut buf = [0u8; 512];
        unsafe { case_fold_simd_neon_inner(bytes, buf.as_mut_ptr(), len) };
        state.write(&buf[..len]);
    } else {
        let mut buf = vec![0u8; len];
        unsafe { case_fold_simd_neon_inner(bytes, buf.as_mut_ptr(), len) };
        state.write(&buf);
    }
}

/// Raw NEON case-fold: lowercase `src[..len]` into `dst`.
///
/// # Safety
/// `dst` must have at least `len` bytes of writable space.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn case_fold_simd_neon_inner(src: &[u8], dst: *mut u8, len: usize) {
    use core::arch::aarch64::*;

    let upper_a = vdupq_n_u8(b'A');
    let upper_z = vdupq_n_u8(b'Z');
    let lower_bit = vdupq_n_u8(0x20);
    let mut i = 0;

    while i + 16 <= len {
        let chunk = vld1q_u8(src.as_ptr().add(i));
        let ge_a = vcgeq_u8(chunk, upper_a);
        let le_z = vcleq_u8(chunk, upper_z);
        let mask = vandq_u8(ge_a, le_z);
        let lowered = vorrq_u8(chunk, vandq_u8(mask, lower_bit));
        vst1q_u8(dst.add(i), lowered);
        i += 16;
    }

    while i < len {
        *dst.add(i) = src[i].to_ascii_lowercase();
        i += 1;
    }
}

impl From<&str> for CaseInsensitiveString {
    #[inline]
    fn from(s: &str) -> Self {
        CaseInsensitiveString(s.into())
    }
}

#[cfg(feature = "compact")]
impl From<CompactString> for CaseInsensitiveString {
    #[inline]
    fn from(s: CompactString) -> Self {
        CaseInsensitiveString(s)
    }
}

impl From<String> for CaseInsensitiveString {
    fn from(s: String) -> Self {
        CaseInsensitiveString(s.into())
    }
}

impl From<&[u8]> for CaseInsensitiveString {
    fn from(s: &[u8]) -> Self {
        CaseInsensitiveString(String::from_utf8_lossy(s).into())
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
        self.0.as_str()
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
        self.0.as_str()
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
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl PartialEq<CaseInsensitiveString> for String {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl PartialEq<&CaseInsensitiveString> for String {
    fn eq(&self, other: &&CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl PartialEq<CaseInsensitiveString> for &String {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl PartialEq<CaseInsensitiveString> for str {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl PartialEq<&CaseInsensitiveString> for str {
    fn eq(&self, other: &&CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl PartialEq<CaseInsensitiveString> for &str {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl PartialEq<CaseInsensitiveString> for &&str {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl<'a> PartialEq<CaseInsensitiveString> for std::borrow::Cow<'a, str> {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl<'a> PartialEq<CaseInsensitiveString> for &std::borrow::Cow<'a, str> {
    fn eq(&self, other: &CaseInsensitiveString) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl PartialEq<String> for &CaseInsensitiveString {
    fn eq(&self, other: &String) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl<'a> PartialEq<std::borrow::Cow<'a, str>> for &CaseInsensitiveString {
    fn eq(&self, other: &std::borrow::Cow<'a, str>) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl PartialOrd for CaseInsensitiveString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CaseInsensitiveString {
    /// Zero-alloc case-insensitive ordering.
    ///
    /// # Examples
    ///
    /// ```
    /// # use case_insensitive_string::CaseInsensitiveString;
    /// assert!(CaseInsensitiveString::from("goodbye") < CaseInsensitiveString::from("hello"));
    /// assert!(CaseInsensitiveString::from("Goodbye") < CaseInsensitiveString::from("hello"));
    /// assert!(CaseInsensitiveString::from("goodbye") < CaseInsensitiveString::from("Hello"));
    /// assert!(CaseInsensitiveString::from("Goodbye") < CaseInsensitiveString::from("Hello"));
    /// ```
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.0.as_bytes();
        let b = other.0.as_bytes();
        let min_len = a.len().min(b.len());

        for i in 0..min_len {
            let ca = a[i].to_ascii_lowercase();
            let cb = b[i].to_ascii_lowercase();
            match ca.cmp(&cb) {
                std::cmp::Ordering::Equal => continue,
                ord => return ord,
            }
        }

        a.len().cmp(&b.len())
    }
}
