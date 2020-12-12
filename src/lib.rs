#[allow(unused_macros)]
macro_rules! indexed_enum {
    (enum $enum_name:ident {
        $($variant_name:ident $(= $index:expr)?,)+
    }) => {
        #[derive(Clone, Copy)]
        enum $enum_name {
            $($variant_name $(= $index)?,)*
        }
        impl Ord for $enum_name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                (*self as i32).cmp(&(*other as i32))
            }
        }
        impl PartialOrd for $enum_name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                (*self as u32).partial_cmp(&(*other as u32))
            }
        }
        impl Eq for $enum_name {}
        impl PartialEq for $enum_name {
            fn eq(&self, other: &Self) -> bool {
                (*self as u32).eq(&(*other as u32))
            }
        }
        impl core::convert::TryFrom<i32> for $enum_name {
            type Error = ();
            fn try_from(integer: i32) -> Result<Self, Self::Error> {
                match integer {
                    $(n if n == $enum_name::$variant_name as i32 => Ok($enum_name::$variant_name),)*
                    _ => Err(()),
                }
            }
        }
    };
}

#[test]
fn variant_to_u32() {
    indexed_enum! {
        enum TestEnum{
            A = 0,
            B,
            C = 4,
        }
    }
    assert_eq!(TestEnum::A as i32, 0);
    assert_eq!(TestEnum::B as i32, 1);
    assert_eq!(TestEnum::C as i32, 4);
}

#[test]
fn u32_to_variant() {
    use core::convert::TryFrom;
    indexed_enum! {
        enum TestEnum {
            A = 0,
            B,
        }
    }
    assert!(TestEnum::try_from(1).unwrap() == TestEnum::B);
}

#[test]
fn variant_ordering() {
    indexed_enum! {
        enum TestEnum{
            A = 0,
            B,
        }
    }
    assert!(TestEnum::A < TestEnum::B);
}
