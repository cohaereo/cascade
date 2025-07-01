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
