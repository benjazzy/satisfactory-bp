use crate::patterns::factory_string::FString;
use std::{io::Write, ops::Deref};

pub trait BPWrite<W: Write> {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error>;
}

// impl<W, T> BPWrite<W> for T
// where
//     W: Write,
//     // for<'a> &'a T: IntoIterator<Item: BPWrite<W>>,
//     T: IntoIterator<Item: BPWrite<W>>,
// {
//     fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
//         for item in self.into_iter() {
//             item.bp_write(writer)?;
//         }
//
//         Ok(())
//     }
// }

// impl<W, L, T> BPWrite<W> for L
// where
//     W: Write,
//     L: AsRef<[u8]>,
// {
//     fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
//         todo!()
//     }
// }

// impl<W, T> BPWrite<W> for &Vec<T>
// where
//     W: Write,
//     for<'a> &'a T: BPWrite<W>,
// {
//     fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
//         for item in self {
//             item.bp_write(writer)?;
//         }
//
//         Ok(())
//     }
// }

impl<W, I> BPWrite<W> for &I
where
    W: Write,
    for<'a> &'a I: IntoIterator,
    for<'a> <&'a I as IntoIterator>::Item: BPWrite<W>,
{
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        for item in self {
            item.bp_write(writer)?;
        }

        Ok(())
    }
}

impl<W: Write> BPWrite<W> for u32 {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_all(self.to_le_bytes().as_slice())
    }
}

impl<W: Write> BPWrite<W> for u64 {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_all(self.to_le_bytes().as_slice())
    }
}

impl<W: Write> BPWrite<W> for f32 {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_all(self.to_le_bytes().as_slice())
    }
}

impl<W: Write> BPWrite<W> for u8 {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_all(&[self])
    }
}

impl<W: Write> BPWrite<W> for &[u8] {
    fn bp_write(self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_all(self)
    }
}
