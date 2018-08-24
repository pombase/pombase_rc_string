extern crate serde;
extern crate serde_json;

use std::fmt;
use std::str;
use std::sync::Arc;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, Add};
use std::cmp::{Ord, Ordering};
use std::borrow::Borrow;

use serde::{Serialize, Deserialize, Serializer, Deserializer,
            de::{Visitor, Unexpected, Error}};


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct RcString {
    rc_string: Arc<String>
}

impl Ord for RcString {
    fn cmp(&self, other: &RcString) -> Ordering {
        self.rc_string.as_ref().cmp(other.rc_string.as_ref())
    }
}

impl RcString {
    pub fn new(s: &str) -> RcString {
        RcString {
            rc_string: Arc::new(s.to_owned())
        }
    }

    pub fn from(s: &str) -> RcString {
        RcString {
            rc_string: Arc::new(s.to_owned())
        }
    }

    pub fn ref_count(&self) -> usize {
        Arc::strong_count(&self.rc_string)
    }

    pub fn as_str(&self) -> &str {
        self.rc_string.as_ref()
    }
}

impl<'a> Add<&'a str> for RcString {
    type Output = String;

    fn add(self, other: &str) -> String {
        self.rc_string.as_ref().clone() + other
    }
}

impl AsRef<str> for RcString {
    fn as_ref(&self) -> &str {
        self.rc_string.as_ref()
    }
}

impl Display for RcString {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.rc_string.fmt(f)
    }
}

impl PartialEq<str> for RcString {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

impl<'a> PartialEq<&'a str> for RcString {
    fn eq(&self, other: &&str) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<String> for RcString {
    fn eq(&self, other: &String) -> bool {
        self.as_ref() == other
    }
}

impl PartialEq<RcString> for String {
    fn eq(&self, other: &RcString) -> bool {
        self == other.rc_string.as_ref()
    }
}

impl<'a> From<&'a str> for RcString {
    fn from(s: &str) -> RcString {
        RcString { rc_string: Arc::new(s.to_owned()) }
    }
}

impl Borrow<String> for RcString {
    fn borrow(&self) -> &String {
        self.rc_string.borrow()
    }
}

impl<'a> Borrow<str> for RcString {
    fn borrow(&self) -> &str {
        self.rc_string.as_ref()
    }
}

impl Deref for RcString {
    type Target = String;

    #[inline]
    fn deref(&self) -> &String {
        self.rc_string.as_ref()
    }
}

impl Serialize for RcString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.rc_string.as_ref())
    }
}

struct RcStringVisitor;

impl<'a> Visitor<'a> for RcStringVisitor {
    type Value = RcString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a borrowed string")
    }

    fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RcString::from(v))
    }

    fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        str::from_utf8(v).map(RcString::from).map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
    }
}

impl<'a> Deserialize<'a> for RcString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_str(RcStringVisitor)
    }
}

impl<'a> From<&'a RcString> for String {
    fn from(s: &RcString) -> String {
        String::from(s.as_ref())
    }
}

impl From<RcString> for String {
    fn from(s: RcString) -> String {
        String::from(s.as_ref())
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json;

    use RcString;

    #[test]
    fn test() {
        let s = RcString::from("test");
        assert!(s == "test");
        assert_eq!(s.ref_count(), 1);
        assert_eq!(s.to_string(), "test");
        let s1 = s.clone();
        assert_eq!(s.ref_count(), 2);
        assert_eq!(s1.to_string(), "test");

        {
            let s2 = s.clone();
            assert_eq!(s.ref_count(), 3);
        }

        assert_eq!(s.ref_count(), 2);

        let s3: &str = &s;
        assert_eq!(s3, "test");

        let mut m: HashMap<RcString, RcString> = HashMap::new();

        m.insert("key".into(), "value".into());

        for key in m.keys() {
            assert_eq!(key, "key");
        }
        for v in m.values() {
            assert_eq!(v, "value");
        }

        let serialized = serde_json::to_string(&m).unwrap();

        assert_eq!(serialized, "{\"key\":\"value\"}");
    }
}

