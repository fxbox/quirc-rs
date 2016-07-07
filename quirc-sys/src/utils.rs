#[macro_export]
macro_rules! try_ue {   // ue = unsafe errno
    ($e:expr) => (if unsafe { $e } < 0 {
            use std::io::Error;
            return Err(Error::last_os_error());
        }
    );
}

#[macro_export]
macro_rules! c_like_enum {
    ( $name: ident { $($variant: ident = $value: expr),+ } ) => {
        #[repr(C)]
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        pub enum $name {
            $($variant = $value,)+
        }

        impl $name {
            pub fn from_u8(value: u8) -> Option<$name> {
                match value {
                    $($value => Some($name::$variant),)+
                    _ => None
                }
            }
        }
    }
}
