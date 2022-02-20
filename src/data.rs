use core::{hash, cmp, fmt};

#[derive(Copy, Clone)]
///Thread name, limited to 16 characters which is common limit for unix systems.
///
///Note that actual limit is 15 characters as 16 includes is needed for terminating character.
///
///Commonly it is valid utf-8 string, but due to potential encoding differences, it is possible
///that name cannot be interpreted as utf-8 string.
///In this case  user is encouraged to use `as_bytes` method and perform conversion by himself.
///
///
///## fmt
///
///- `Debug` trait outputs content as 16 bytes.
///
///- `Display` trait outputs content as string, if possible, otherwise fallbacks to byte slice.
pub struct ThreadName {
    name: [u8; 16]
}

impl ThreadName {
    #[inline(always)]
    ///Creates new empty string name
    pub const fn new() -> Self {
        Self {
            name: [0; 16]
        }
    }

    #[inline(always)]
    ///Initializes name from buffer.
    pub const fn name(name: [u8; 16]) -> Self {
        Self {
            name
        }
    }

    #[inline]
    ///Returns name as byte slice
    pub fn as_bytes(&self) -> &[u8] {
        let mut idx = 0;
        while idx < self.name.len() {
            if self.name[idx] == 0 {
                return &self.name[..idx]
            }
            idx += 1;
        }

        self.name.as_slice()
    }

    #[inline]
    ///Returns name as string, checking whether it is valid utf-8 before.
    ///
    ///In case of underlying name not to be valid string, returns byte slice with content.
    ///
    ///On windows it never fails.
    pub fn as_str(&self) -> Result<&str, &[u8]> {
        #[cold]
        #[inline(never)]
        fn fail<T>(res: T) -> T {
            res
        }

        let bytes = self.as_bytes();
        match core::str::from_utf8(bytes) {
            Ok(res) => Ok(res),
            Err(_) => fail(Err(bytes)),
        }
    }
}

impl AsRef<[u8]> for ThreadName {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl core::borrow::Borrow<[u8]> for ThreadName {
    #[inline(always)]
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Eq for ThreadName {}

impl PartialEq<ThreadName> for ThreadName {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<ThreadName> for &str {
    #[inline(always)]
    fn eq(&self, other: &ThreadName) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<ThreadName> for str {
    #[inline(always)]
    fn eq(&self, other: &ThreadName) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<ThreadName> for &[u8] {
    #[inline(always)]
    fn eq(&self, other: &ThreadName) -> bool {
        *self == other.as_bytes()
    }
}

impl PartialEq<ThreadName> for [u8] {
    #[inline(always)]
    fn eq(&self, other: &ThreadName) -> bool {
        self == other.as_bytes()
    }
}

impl PartialEq<str> for ThreadName {
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<&str> for ThreadName {
    #[inline(always)]
    fn eq(&self, other: &&str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<[u8]> for ThreadName {
    #[inline(always)]
    fn eq(&self, other: &[u8]) -> bool {
        self.as_bytes() == other
    }
}

impl PartialEq<&[u8]> for ThreadName {
    #[inline(always)]
    fn eq(&self, other: &&[u8]) -> bool {
        self.as_bytes() == *other
    }
}

impl cmp::Ord for ThreadName {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

impl PartialOrd for ThreadName {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl hash::Hash for ThreadName {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.as_bytes().hash(hasher)
    }
}

impl fmt::Debug for ThreadName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.name, fmt)
    }
}

impl fmt::Display for ThreadName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.as_str() {
            Ok(name) => fmt.write_str(name),
            Err(bytes) => fmt::Debug::fmt(bytes, fmt)
        }
    }
}
