//! IO objects that can be shortened.
//!
//! See the [`Truncate`] trait.

use std::{
    cmp,
    fs::File,
    io::{Cursor, Error, ErrorKind},
};

/// A trait for IO objects that can be shortened.
///
/// See the documentation comments on individual implementations for some potentially important
/// notes on their specific behaviors.
pub trait Truncate {
    /// Truncate the object to the given new length in bytes.
    ///
    /// The behavior when `new_len` is larger than the current length of the object is unspecified.
    /// Implementations may choose to panic or extend the data in some way.
    ///
    /// # Example
    ///
    /// ```
    /// # use io_truncate::Truncate;
    /// let mut v: &[u8] = &[0, 1, 2, 3];
    /// v.truncate(3).unwrap();
    /// assert_eq!(v, &[0, 1, 2]);
    /// ```
    fn truncate(&mut self, new_len: usize) -> Result<(), Error>;
}

impl Truncate for File {
    /// Delegates to [`File::set_len`].
    fn truncate(&mut self, new_len: usize) -> Result<(), Error> {
        self.set_len(new_len as u64)
    }
}

impl Truncate for Vec<u8> {
    /// Shortens the `Vec` or returns an error if the length would be larger than the current
    /// length.
    fn truncate(&mut self, new_len: usize) -> Result<(), Error> {
        if new_len <= self.len() {
            self.truncate(new_len);
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "tried to truncate to greater length ({} > {})",
                    new_len,
                    self.len()
                ),
            ))
        }
    }
}

impl<'a> Truncate for &'a [u8] {
    /// Shortens the slice or returns and error if the length would be larger than the current
    /// length.
    fn truncate(&mut self, new_len: usize) -> Result<(), Error> {
        if new_len <= self.len() {
            *self = &self[..new_len];
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "tried to truncate to greater length ({} > {})",
                    new_len,
                    self.len()
                ),
            ))
        }
    }
}

impl<T> Truncate for Cursor<T>
where
    T: Truncate,
{
    /// Delegates to the contained [`Truncate`] impl. The cursor will be moved to the end of the
    /// data if it lies in the truncated area.
    fn truncate(&mut self, new_len: usize) -> Result<(), Error> {
        self.get_mut().truncate(new_len)?;
        self.set_position(cmp::min(new_len as u64, self.position()));
        Ok(())
    }
}

impl<T> Truncate for &mut T
where
    T: Truncate,
{
    fn truncate(&mut self, new_len: usize) -> Result<(), Error> {
        (**self).truncate(new_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Seek, SeekFrom, Write};

    #[test]
    fn vec() {
        let mut v: Vec<u8> = vec![0, 1, 2, 3];

        // Need to call like this in order to not conflict with the inherent method.
        Truncate::truncate(&mut v, 3).unwrap();
        assert_eq!(v, &[0, 1, 2]);

        // Error
        let e = Truncate::truncate(&mut v, 4).unwrap_err();
        assert_eq!(e.kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn slice() {
        let mut v: &[u8] = &[0, 1, 2, 3];

        v.truncate(3).unwrap();
        assert_eq!(v, &[0, 1, 2]);

        // Error
        let e = v.truncate(4).unwrap_err();
        assert_eq!(e.kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn cursor() {
        let mut v: Cursor<&[u8]> = Cursor::new(&[0, 1, 2, 3]);

        v.set_position(4); // end of data
        v.truncate(3).unwrap();
        assert_eq!(v.get_ref(), &[0, 1, 2]);
        assert_eq!(v.position(), 3);

        // Error
        let e = v.truncate(4).unwrap_err();
        assert_eq!(e.kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn file() {
        let mut f = tempfile::tempfile().unwrap();
        f.write_all(&[0, 1, 2, 3]).unwrap();
        f.seek(SeekFrom::Start(0)).unwrap();

        f.truncate(3).unwrap();
        assert_eq!(f.seek(SeekFrom::End(0)).unwrap(), 3);

        // File::set_len works with longer values too
    }
}
