use std::io::Read;

/// ```text
/// bitfield! {
/// struct MyFlags : u32 {
///     is_active: bool @ 0x01,
///     kind: KindEnum @ 0x06 >> 1,
/// }
/// ```
#[macro_export]
macro_rules! bitfield {
    (
        $(pub)? struct $name:ident : $repr_type:ident { $(
        $(pub)?
        $(flag $fgetter_name:ident : bool @ $flag_mask:literal)?
        $(enum $egetter_name:ident : $kind:ident @ $enum_mask:literal >> $enum_shift:literal $(as $cast_enum:ident)?)?
    ),* }) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub $repr_type);

        impl $name {
            $(
                $(
                    pub fn $fgetter_name(&self) -> bool {
                        (self.0 & $flag_mask) != 0
                    }
                )?

                $(
                    pub fn $egetter_name(&self) -> $kind {
                        (((self.0 & $enum_mask) >> $enum_shift) $(as $cast_enum)?).try_into().expect(
                            concat!(
                                stringify!($name),
                                "::",
                                stringify!($egetter_name),
                                " failed to convert to ",
                                stringify!($kind)
                            )
                        )
                    }
                )?
            )*
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    $(
                        $(.field(stringify!($fgetter_name), &self.$fgetter_name()))?
                        $(.field(stringify!($egetter_name), &self.$egetter_name()))?
                    )*
                    .field("value", &self.0)
                    .finish()
            }
        }

        impl binrw::BinRead for $name {
            type Args<'a> = ();

            fn read_options<R: std::io::Read + std::io::Seek>(reader: &mut R, endian: binrw::Endian, args: Self::Args<'_>) -> binrw::BinResult<Self> {
                let value: $repr_type = binrw::BinRead::read_options(reader, endian, args)?;
                Ok($name(value))
            }
        }
    };
}

pub trait ReadExt {
    fn read_compressed_u32(&mut self) -> binrw::BinResult<u32>;
}

impl<T> ReadExt for T
where
    T: Read,
{
    fn read_compressed_u32(&mut self) -> binrw::BinResult<u32> {
        let mut result = 0;
        // Varint encoding
        // For unsigned integers:
        //   - If the value lies between 0 (0x00) and 127 (0x7F), inclusive, encode as a one-byte integer (bit 7 is clear, value held in bits 6 through 0)
        //   - If the value lies between 28 (0x80) and 214-1 (0x3FFF), inclusive, encode as a 2-byte integer with bit 15 set, bit 14 clear (value held in bits 13 through 0)
        //   - Otherwise, encode as a 4-byte integer, with bit 31 set, bit 30 set, bit 29 clear (value held in bits 28 through 0)
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        let first_byte = buf[0];
        if first_byte & 0x80 == 0 {
            // One-byte integer (0-127)
            result = first_byte as u32;
        } else if first_byte & 0xC0 == 0x80 {
            // Two-byte integer (128-16383)
            let mut data = [0u8; 2];
            self.read_exact(&mut data)?;
            let second_byte = data[0];
            result = (((first_byte & 0x3F) as u32) << 8) | (second_byte as u32);
        } else if first_byte & 0xE0 == 0xC0 {
            let mut data = [0u8; 3];
            self.read_exact(&mut data)?;
            // Four-byte integer
            let second_byte = data[0];
            let third_byte = data[1];
            let fourth_byte = data[2];
            result = (((first_byte & 0x1F) as u32) << 24)
                | ((second_byte as u32) << 16)
                | ((third_byte as u32) << 8)
                | (fourth_byte as u32);
        }

        Ok(result)
    }
}
