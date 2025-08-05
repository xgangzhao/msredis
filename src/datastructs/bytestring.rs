use std::convert::TryFrom;
use std::collections::HashSet;
use std::str::Bytes;


/// A simple byte string wrapper.
///
/// This struct represents a sequence of bytes, similar to a string but without
/// the UTF-8 encoding requirement. It is useful for handling raw binary data.
///
/// # Fields
///
/// * `buf` - Internal buffer. A `Vec<u8>` that stores the byte sequence.
///
#[derive(Debug, PartialEq, Eq)]
pub struct ByteString {
    buf: Vec<u8>,
}

impl ByteString {
    /// Creates a new `ByteString` from a given string.
    ///
    /// # Arguments
    ///
    /// * `buf` - The initial string
    /// 
    pub fn new(s: &str) -> Self {
        return ByteString{buf: s.as_bytes().to_vec()};
    }

    /// Creates a new `ByteString` from a given ByteString.
    ///
    /// # Arguments
    ///
    /// * `bs` - A ByteString
    ///
    pub fn new_from(bs: &ByteString) -> Self {
        return ByteString{buf: bs.buf.clone()};
    }

    /// Creates a new `ByteString` from a given bytes stream
    ///
    /// # Arguments
    ///
    /// * `ss` - A bytes stream
    ///
    pub fn new_from_bytes(ss: &[u8]) -> Self {
        return ByteString{buf: ss.to_vec()}
    }

    /// Duplicate a `ByteString` from itself.
    ///
    /// # Arguments
    ///
    /// None
    ///
    pub fn dup(&self) -> ByteString {
        return ByteString{buf: self.buf.clone()};
    }

    /// The number of elements in the ByteString.
    ///
    /// # Arguments
    ///
    /// None
    ///
    pub fn len(&self) -> usize {
        return self.buf.len();
    }

    /// The total number of elements the ByteString can hold without reallocating.
    /// 
    /// # Arguments
    ///
    /// None
    ///
    pub fn capacity(&self) -> usize {
        return self.buf.capacity();
    }

    /// Grow the ByteString to have the specified length in place. Bytes that were not part of
    /// the original length of the sds will be set to zero. 
    /// If the specified length is smaller than the current length, no operation is performed.
    /// 
    /// # Arguments
    ///
    /// * `len` - A specified length
    ///
    pub fn grow_zero(&mut self, len: usize) {
        if len > self.buf.len() {
            self.buf.resize(len, 0x00);
        }
    }

    /// Append a specified ByteString to the internal buffer in place. 
    /// 
    /// # Arguments
    ///
    /// * `bs` - A specified ByteString
    ///
    pub fn cat(&mut self, bs: &ByteString) {
        self.buf.extend(bs.buf.clone());
    }

    /// Turn the ByteString into a smaller (or equal) ByteString containing only the substring specified by the `start` and `end` indexes.
    /// start and end can be negative, where -1 means the last character of the ByteString, and so forth. Modify in-place.
    /// The caller **must** ensure that the range is valid.
    /// 
    /// # Arguments
    ///
    /// * `start` - start index
    /// * `end` - end index
    ///
    pub fn range(&mut self, start: i64, end: i64) {
        let sz = self.buf.len();
        let mut s_tmp: i64 = start;
        let mut e_tmp: i64 = end;
        if sz == 0 {
            return;
        }
        if s_tmp < 0 {
            s_tmp = sz as i64 + s_tmp;
            if s_tmp < 0 {
                s_tmp = 0;
            }
        }
        if e_tmp < 0 {
            e_tmp = sz as i64 + e_tmp + 1;
            if e_tmp < 0 {
                e_tmp = 0;
            }
        }
        if s_tmp as usize >= sz {
            s_tmp = 0;
        }
        if e_tmp as usize > sz {
            e_tmp = sz as i64;
        }
        let vec = self.buf[s_tmp as usize..e_tmp as usize].to_vec();
        self.buf = vec;
    }

    /// Chear the ByteString.
    /// 
    /// # Arguments
    ///
    /// None
    ///
    pub fn clear(&mut self) {
        self.buf.clear();
    }

    /// Remove the part of the string from left and from right composed just of contiguous bytes 
    /// found in `cset`
    /// 
    /// # Arguments
    ///
    /// * `cset` - The specified bytes
    ///
    pub fn trim(&self, cset: &[u8]) -> Self{
        let mut i: usize = 0;
        let mut j: usize = self.buf.len() - 1;
        let vec = self.buf.clone();
        let set: HashSet<u8> = cset.iter().cloned().collect();
        for (idx, byte) in self.buf.iter().enumerate() {
            if !set.contains(byte) {
                i = idx;
                break;
            }
        }
        for byte in self.buf.iter().rev() {
            if !set.contains(byte) {
                break;
            }
            j -= 1;
        }

        return ByteString { buf: vec[i..j+1].to_vec() };
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytestring::ByteString;

    #[test]
    fn test_new() {
        let bs = ByteString::new("hello, world");
        assert_eq!(bs.buf, vec![104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100]);
    }

    #[test]
    fn test_new_from() {
        let bs = ByteString::new("hello, world");
        let bs2 = ByteString::new_from(&bs);
        assert_eq!(bs, bs2);
    }

    #[test]
    fn test_new_from_bytes() {
        let buf: Vec<u8> = vec![104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100];
        let bs = ByteString::new("hello, world");
        let bs2 = ByteString::new_from_bytes(&buf);
        assert_eq!(bs, bs2);
    }

    #[test]
    fn test_dup() {
        let bs = ByteString::new("hello, world");
        let bs_dup = bs.dup();
        assert_eq!(bs.buf, bs_dup.buf);
    }

    #[test]
    fn test_grow_zero() {
        let mut bs = ByteString::new("");
        bs.grow_zero(8);
        assert_eq!(bs.buf, vec![0; 8]);
    }

    #[test]
    fn test_cat() {
        let mut a = ByteString::new("hello,");
        let another = ByteString::new(" world");
        let target = ByteString::new("hello, world");
        a.cat(&another);
        assert_eq!(a, target);
        assert_eq!(a.buf, target.buf);
    }

    #[test]
    fn test_range() {
        let target = ByteString::new("hello, world");
        let mut a = ByteString::new("aahello, world");
        let target2 = ByteString::new("ello, worl");
        a.range(2, -1);
        assert_eq!(target.buf, a.buf);
        a.range(20, -1);
        assert_eq!(target.buf, a.buf);  // hello, world
        a.range(1, -2);
        assert_eq!(target2.buf, a.buf);
    }

    #[test]
    fn test_clear() {
        let mut target = ByteString::new("hello, world");
        target.clear();
        assert_eq!(target.buf.len(), 0);
    }

    #[test]
    fn test_trim() {
        let bs = ByteString::new("hhheeello, worlllddd");
        let cset: Vec<u8> = vec![104, 101, 108, 100];  // h, e, l, d
        let x = bs.trim(&cset);
        let y = ByteString::new("o, wor");
        assert_eq!(x.buf, y.buf);
    }
}