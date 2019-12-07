use byteorder::{ReadBytesExt, LE};
use std::convert::TryInto;
use std::io::{Error, ErrorKind, Read, Result};

/// Read a 2-byte integer that uses -1 as an "absent" value.
///
/// ## Example
///
/// ```rust
/// use genie_support::read_opt_u16;
///
/// let mut minus_one = std::io::Cursor::new(vec![0xFF, 0xFF]);
/// let mut zero = std::io::Cursor::new(vec![0x00, 0x00]);
///
/// assert_eq!(read_opt_u16(&mut minus_one).unwrap(), None);
/// assert_eq!(read_opt_u16(&mut zero).unwrap(), Some(0));
/// ```
#[inline]
pub fn read_opt_u16<R: Read>(input: &mut R) -> Result<Option<u16>> {
    let v = input.read_i16::<LE>()?;
    if v == -1 {
        return Ok(None);
    }
    Ok(Some(
        v.try_into().map_err(|e| Error::new(ErrorKind::Other, e))?,
    ))
}

/// Read a 4-byte integer that uses -1 as an "absent" value.
///
/// ## Example
///
/// ```rust
/// use genie_support::read_opt_u32;
///
/// let mut minus_one = std::io::Cursor::new(vec![0xFF, 0xFF, 0xFF, 0xFF]);
/// let mut one = std::io::Cursor::new(vec![0x01, 0x00, 0x00, 0x00]);
///
/// assert_eq!(read_opt_u32(&mut minus_one).unwrap(), None);
/// assert_eq!(read_opt_u32(&mut one).unwrap(), Some(1));
/// ```
#[inline]
pub fn read_opt_u32<R: Read>(input: &mut R) -> Result<Option<u32>> {
    let v = input.read_i32::<LE>()?;
    if v < 0 {
        return Ok(None);
    }
    Ok(Some(
        v.try_into().map_err(|e| Error::new(ErrorKind::Other, e))?,
    ))
}