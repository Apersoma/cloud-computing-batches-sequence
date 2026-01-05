pub trait Signed {
    type Unsigned: Unsigned;
}

pub trait Unsigned {
    type Signed: Signed;
}

macro_rules! signed_unsigned {
    ($u:ty, $i:ty) => {
        impl Signed for $i {
            type Unsigned = $u;
        }
        impl Unsigned for $u {
            type Signed = $i;
        }
    };
}


signed_unsigned!(u8, i8);
signed_unsigned!(u16, i16);
signed_unsigned!(u32, i32);
signed_unsigned!(u64, i64);
signed_unsigned!(usize, isize);
signed_unsigned!(u128, i128);

#[macro_export]
macro_rules! signed {
    ($t:ty) => {
        <$t as Unsigned>::Signed
    };
}

#[macro_export]
macro_rules! unsigned {
    ($t:ty) => {
        <t as Signed>::Unsigned
    };
}