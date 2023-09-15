use serde::ser::SerializeStruct;
use serde::Serialize;
use serde::Serializer;

/*
use core::fmt;
use serde::de::DeserializeSeed;
use serde::Deserializer;
use serde::Deserialize;
use serde::de;
*/

#[repr(transparent)]
pub struct FixedSizeByteArray<const N: usize> {
    inner: FixedSizeByteArrayInner<N>,
}

impl<const N: usize> From<[u8; N]> for FixedSizeByteArray<N> {
    fn from(array: [u8; N]) -> FixedSizeByteArray<N> {
        FixedSizeByteArray {
            inner: FixedSizeByteArrayInner {
                array,
            },
        }
    }
}

#[repr(transparent)]
struct FixedSizeByteArrayInner<const N: usize> {
    array: [u8; N],
}

pub static TOKEN: &str = "$postcard::private::FixedSizeByteArray";

impl<const N: usize> Serialize for FixedSizeByteArray<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct(TOKEN, 1)?;
        s.serialize_field(TOKEN, &self.inner)?;
        s.end()
    }
}

impl<const N: usize> Serialize for FixedSizeByteArrayInner<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.array)
    }
}

/*
impl<'de: 'a, 'a, const N: usize> Deserialize<'de> for &'a FixedSizeByteArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReferenceVisitor;

        impl<'de, const N: usize> de::Visitor<'de> for ReferenceVisitor {
            type Value = &'de FixedSizeByteArray<N>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "any valid JSON value")
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let value = visitor.next_key::<FixedSizeByteArrayKey>()?;
                if value.is_none() {
                    return Err(de::Error::invalid_type(Unexpected::Map, &self));
                }
                visitor.next_value_seed(ReferenceFromBytes)
            }
        }

        deserializer.deserialize_newtype_struct(TOKEN, ReferenceVisitor)
    }
}

impl<'de, const N: usize> Deserialize<'de> for FixedSizeByteArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de, const N: usize> de::Visitor<'de> for Visitor {
            type Value = FixedSizeByteArray<N>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "any valid JSON value")
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let value = visitor.next_key::<FixedSizeByteArrayKey>()?;
                if value.is_none() {
                    return Err(de::Error::invalid_type(Unexpected::Map, &self));
                }
                visitor.next_value_seed(FromBytes)
            }
        }

        deserializer.deserialize_newtype_struct(TOKEN, Visitor)
    }
}

struct FixedSizeByteArrayKey;

impl<'de> Deserialize<'de> for FixedSizeByteArrayKey {
    fn deserialize<D>(deserializer: D) -> Result<FixedSizeByteArrayKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> de::Visitor<'de> for FieldVisitor {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("fixed size byte array")
            }

            fn visit_str<E>(self, s: &str) -> Result<(), E>
            where
                E: de::Error,
            {
                if s == TOKEN {
                    Ok(())
                } else {
                    Err(de::Error::custom("unexpected fixed size byte array"))
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)?;
        Ok(FixedSizeByteArrayKey)
    }
}

struct FromBytes;

impl<'de, const N: usize> DeserializeSeed<'de> for FromBytes {
    type Value = FixedSizeByteArray<N>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(self)
    }
}

impl<'de, const N: usize> de::Visitor<'de> for FromBytes {
    type Value = FixedSizeByteArray<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("raw value")
    }

    fn visit_bytes<E>(self, s: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(FixedSizeByteArray::from_owned(s.to_owned().into_boxed_str()))
    }
}
*/

#[cfg(test)]
mod tests {
    use super::FixedSizeByteArray;

    #[test]
    fn test_byte_array() {
        let empty = FixedSizeByteArray::from([]);
        let mut buf = [0; 32];
        let serialized = crate::to_slice(&empty, &mut buf).unwrap();
        assert_eq!(serialized, &[]);

        let single = FixedSizeByteArray::from([0x12]);
        let mut buf = [0; 32];
        let serialized = crate::to_slice(&single, &mut buf).unwrap();
        assert_eq!(serialized, &[0x12]);

        let five_bytes = FixedSizeByteArray::from([0x12, 0x34, 0x56, 0x78, 0x90]);
        let mut buf = [0; 32];
        let serialized = crate::to_slice(&five_bytes, &mut buf).unwrap();
        assert_eq!(serialized, &[0x12, 0x34, 0x56, 0x78, 0x90]);
    }
}
