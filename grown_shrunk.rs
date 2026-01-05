
pub trait DoubleWidth {
    type HalfWidth: HalfWidth;
}

pub trait HalfWidth {
    type DoubleWidth: DoubleWidth;
}

macro_rules! double_halved {
    ($d:ty, $h:ty) => {
        impl DoubleWidth for $d {
            type HalfWidth = $h;
        }
        impl HalfWidth for $h {
            type DoubleWidth = $d;
        }
    };
}


double_halved!(u16, u8);
double_halved!(u32, u16);
double_halved!(u64, u32);
double_halved!(u128, u64);
double_halved!(i16, i8);
double_halved!(i32, i16);
double_halved!(i64, i32);
double_halved!(i128, i64);


#[macro_export]
macro_rules! double_width {
    ($t:ty) => {
        <$t as HalfWidth>::DoubleWidth
    };
}

#[macro_export]
macro_rules! half_width {
    ($t:ty) => {
        <t as DoubleWidth>::HalfWidth
    };
}