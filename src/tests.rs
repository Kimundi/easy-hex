use serde::{de::DeserializeOwned, Serialize};
use serde_derive::{Deserialize, Serialize};

use super::*;

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
        assert_eq!(format!("{}", $hex), "0199ff");
        assert_eq!(to_json($hex.clone()), r#"{"data":"0199ff"}"#);

        let r: &$T = &$hex;
        let _rh: &$H = $H::from_ref(r);

        let m: &mut $T = &mut $hex;
        let _mh: &mut $H = $H::from_mut(m);

        let _i: $T = $hex.clone().into_inner();

        let as_r: &[u8] = $hex.as_ref();
        assert_eq!(as_r, [1, 0x99, 0xff]);
    }};
}
macro_rules! test_make {
    ($T:ident, $H:ident, $hex:ident) => {{
        let hex2 = from_json(r#"{"data":"0199ff"}"#);
        assert_eq!($hex, hex2);

        let _n: $H = $H::default();
    }};
}

make_cases! {
    test_vec<'a, T = Vec<u8>, H = Hex<Vec<u8>>>(vec![1, 0x99, 0xff], |mut hex: H| {
        test_convert!(T, H, hex);
        test_make!(T, H, hex);
    });
    test_array<'a, T = [u8; 3], H = Hex<[u8; 3]>>([1, 0x99, 0xff], |mut hex: H| {
        test_convert!(T, H, hex);
        test_make!(T, H, hex);
    });
    test_slice<'a, T = &'a [u8], H = Hex<&'a [u8]>>(&[1, 0x99, 0xff], |mut hex: H| {
        test_convert!(T, H, hex);
    });
    test_array_ref<'a, T = &'a [u8; 3], H = Hex<&'a [u8; 3]>>(&[1, 0x99, 0xff], |mut hex: H| {
        test_convert!(T, H, hex);
    });
    test_vec_ref<'a, T = &'a Vec<u8>, H = Hex<&'a Vec<u8>>>(&vec![1, 0x99, 0xff], |mut hex: H| {
        test_convert!(T, H, hex);
    });
}

// TODO: test
// only pod or other arrays
// let _z ) bytemuck::zero()

/*
TODO: write tests for all cases
- bare slice unsized?

*/

#[derive(Serialize, Deserialize)]
struct TestJson<V> {
    data: V,
}

pub(crate) fn to_json<V>(data: V) -> String
where
    V: Serialize,
{
    serde_json::to_string(&TestJson { data }).unwrap()
}

pub(crate) fn from_json<V>(data: &str) -> V
where
    V: DeserializeOwned,
{
    serde_json::from_str::<TestJson<V>>(data).unwrap().data
}
