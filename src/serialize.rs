use serde::{Serialize, Serializer};

use crate::{encode::fast_serialize, Hex};

impl<T, const U: bool> Serialize for Hex<T, U>
where
    T: AsRef<[u8]>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        fast_serialize::<_, _, U>(&self.0, |s| serializer.serialize_str(s))
    }
}
