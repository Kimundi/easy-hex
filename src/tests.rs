use super::*;

#[cfg(feature = "serde")]
use ::serde::{de::DeserializeOwned, Serialize};
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

use generic_array::{typenum::U3, GenericArray};

macro_rules! make_cases {
    ($($name:ident<
        $l:lifetime,
        $base_ty_name:ident = $base_ty:ty,
        $hex_ty_name:ident = $hex_ty:ty
    >(
        $val:expr,
        $test_fn:expr
    );)*) => {
        $(
            #[test]
            fn $name() {
                type $hex_ty_name<$l> = $hex_ty;
                type $base_ty_name<$l> = $base_ty;

                let base: $base_ty_name  = $val;
                let hex: $hex_ty_name = base.into();
                let test_fn = $test_fn;
                test_fn(hex);
            }
        )*
    }
}

macro_rules! test_convert {
    ($T:ident, $H:ident, $hex:ident) => {{
        assert_eq!(format!("{}", $hex).to_lowercase(), "0199ff");
        #[cfg(feature = "serde")]
        assert_eq!(to_json(&$hex).to_lowercase(), r#"{"data":"0199ff"}"#);

        let as_r: &[u8] = $hex.as_ref();
        assert_eq!(as_r, [1, 0x99, 0xff]);
    }};
}

macro_rules! test_owned_convert {
    ($T:ident, $H:ident, $hex:ident) => {{
        test_convert!($T, $H, $hex);

        let r: &$T = &$hex;
        let _rh: &$H = r.into();

        let m: &mut $T = &mut $hex;
        let _mh: &mut $H = m.into();

        let _i: $T = $hex.0;
    }};
}
macro_rules! test_make {
    ($T:ident, $H:ident, $hex:ident) => {{
        #[cfg(feature = "serde")]
        let hex2 = from_json(r#"{"data":"0199ff"}"#);
        #[cfg(feature = "serde")]
        assert_eq!($hex, hex2);

        let _n: $H = $H::default();
    }};
}

macro_rules! make_group {
    ($group:ident, $Hex:ident) => {
        mod $group {
            use super::*;

            make_cases! {
                test_vec<'a, T = Vec<u8>, H = $Hex<Vec<u8>>>(vec![1, 0x99, 0xff], |mut hex: H| {
                    test_make!(T, H, hex);
                    test_owned_convert!(T, H, hex);
                });

                test_boxed_slice<'a, T = Box<[u8]>, H = $Hex<Box<[u8]>>>(vec![1, 0x99, 0xff].into_boxed_slice(), |mut hex: H| {
                    test_make!(T, H, hex);
                    test_owned_convert!(T, H, hex);
                });

                test_array<'a, T = [u8; 3], H = $Hex<[u8; 3]>>([1, 0x99, 0xff], |mut hex: H| {
                    test_make!(T, H, hex);
                    test_owned_convert!(T, H, hex);
                    let zeroed: H = H::zeroed();
                    assert_eq!(zeroed, $Hex([0, 0, 0]));
                });

                test_slice<'a, T = &'a [u8], H = $Hex<&'a [u8]>>(&[1, 0x99, 0xff], |mut hex: H| {
                    test_owned_convert!(T, H, hex);
                });

                test_array_ref<'a, T = &'a [u8; 3], H = $Hex<&'a [u8; 3]>>(&[1, 0x99, 0xff], |mut hex: H| {
                    test_owned_convert!(T, H, hex);
                });

                test_vec_ref<'a, T = &'a Vec<u8>, H = $Hex<&'a Vec<u8>>>(&vec![1, 0x99, 0xff], |mut hex: H| {
                    test_owned_convert!(T, H, hex);
                });

                test_ref_vec<'a, T = &'a Vec<u8>, H = &'a $Hex<Vec<u8>>>(&vec![1, 0x99, 0xff], |hex: H| {
                    test_convert!(T, H, hex);
                });

                test_slice_mut<'a, T = &'a mut [u8], H = $Hex<&'a mut [u8]>>(&mut [1, 0x99, 0xff], |mut hex: H| {
                    test_owned_convert!(T, H, hex);
                });

                test_array_ref_mut<'a, T = &'a mut [u8; 3], H = $Hex<&'a mut [u8; 3]>>(&mut [1, 0x99, 0xff], |mut hex: H| {
                    test_owned_convert!(T, H, hex);
                });

                test_vec_ref_mut<'a, T = &'a mut Vec<u8>, H = $Hex<&'a mut Vec<u8>>>(&mut vec![1, 0x99, 0xff], |mut hex: H| {
                    test_owned_convert!(T, H, hex);
                });

                test_mut_vec<'a, T = &'a mut Vec<u8>, H = &'a mut $Hex<Vec<u8>>>(&mut vec![1, 0x99, 0xff], |hex: H| {
                    test_convert!(T, H, hex);
                });

                test_ref_slice<'a, T = &'a [u8], H = &'a $Hex<[u8]>>(&[1, 0x99, 0xff], |hex: H| {
                    test_convert!(T, H, hex);
                });

                test_gen_array<'a, T = GenericArray<u8, U3>, H = $Hex<GenericArray<u8, U3>>>([1, 0x99, 0xff].into(), |mut hex: H| {
                    test_owned_convert!(T, H, hex);
                    // TODO: After the GA 1.0 release we can probably provide a PR
                    // to make this work?
                    // test_make!(T, H, hex);
                });
            }
        }
    }
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
struct TestJson<V> {
    data: V,
}

#[cfg(feature = "serde")]
pub(crate) fn to_json<V>(data: &V) -> String
where
    V: Serialize + ?Sized,
{
    serde_json::to_string(&TestJson { data }).unwrap()
}

#[cfg(feature = "serde")]
pub(crate) fn from_json<V>(data: &str) -> V
where
    V: DeserializeOwned,
{
    serde_json::from_str::<TestJson<V>>(data).unwrap().data
}

make_group!(lower, Hex);
make_group!(upper, UpperHex);
