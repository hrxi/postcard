use serde::{ser, Serialize};

use crate::error::{Error, Result};
use heapless::Vec;
use byteorder::{LittleEndian, ByteOrder};

// Should be 5 for u32, and 10 for u64
const VARINT_MAX_SZ: usize = core::mem::size_of::<usize>() + (core::mem::size_of::<usize>() / 4);

const fn new_varint_buf() -> [u8; VARINT_MAX_SZ] {
    [0u8; VARINT_MAX_SZ]
}

// AJM!
fn usize_to_varint(mut value: usize, out: &mut [u8; VARINT_MAX_SZ]) -> &mut [u8] {
    for i in 0..VARINT_MAX_SZ {
        out[i] = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            out[i] |= 0x80;
        } else {
            return &mut out[..=i]
        }
    }
    debug_assert_eq!(value, 0);
    &mut out[..]
}

pub struct Serializer<B>
where
    B: heapless::ArrayLength<u8>
{
    output: Vec<u8, B>
}

pub struct SerializeStruct<'a, B>
where
    B: heapless::ArrayLength<u8>,
{
    de: &'a mut Serializer<B>,
}

pub struct SerializeSeq<'a, B>
where
    B: heapless::ArrayLength<u8>,
{
    de: &'a mut Serializer<B>,
}

pub struct SerializeTuple<'a, B>
where
    B: heapless::ArrayLength<u8>,
{
    de: &'a mut Serializer<B>,
}

pub struct SerializeStructVariant<'a, B>
where
    B: heapless::ArrayLength<u8>,
{
    de: &'a mut Serializer<B>,
}

pub struct SerializeTupleStruct<'a, B>
where
    B: heapless::ArrayLength<u8>,
{
    de: &'a mut Serializer<B>,
}

pub struct SerializeTupleVariant<'a, B>
where
    B: heapless::ArrayLength<u8>,
{
    de: &'a mut Serializer<B>,
}



// By convention, the public API of a Serde serializer is one or more `to_abc`
// functions such as `to_vec`, `to_bytes`, or `to_writer` depending on what
// Rust types the serializer is able to produce as output.
//
// This basic serializer supports only `to_vec`.
pub fn to_vec<B, T>(value: &T) -> Result<Vec<u8, B>>
where
    T: Serialize + ?Sized,
    B: heapless::ArrayLength<u8>,
{
    let mut serializer = Serializer {
        output: Vec::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a, B> Serializer<B>
where
    B: heapless::ArrayLength<u8>
{
    fn push_usize(&mut self, value: usize) -> Result<()> {
        let mut sz_buf = new_varint_buf();
        let sz = usize_to_varint(value, &mut sz_buf);
        self.output.extend_from_slice(sz).map_err(|_| Error::ToDo)
    }
}

impl<'a, B> ser::Serializer for &'a mut Serializer<B>
where
    B: heapless::ArrayLength<u8>
{
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = SerializeSeq<'a, B>;
    type SerializeTuple = SerializeTuple<'a, B>;
    type SerializeTupleStruct = SerializeTupleStruct<'a, B>;
    type SerializeTupleVariant = SerializeTupleVariant<'a, B>;
    type SerializeMap = Self;
    type SerializeStruct = SerializeStruct<'a, B>;
    type SerializeStructVariant = SerializeStructVariant<'a, B>;

    // Here we go with the simple methods. The following 12 methods receive one
    // of the primitive types of the data model and map it to JSON by appending
    // into the output string.
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_u8(if v { 1 } else { 0 })
    }

    // JSON does not distinguish between different sizes of integers, so all
    // signed integers will be serialized the same and all unsigned integers
    // will be serialized the same. Other formats, especially compact binary
    // formats, may need independent logic for the different sizes.
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_u8(v.to_le_bytes()[0])
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.output
            .extend_from_slice(&v.to_le_bytes())
            .map_err(|_| Error::ToDo)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.output
            .extend_from_slice(&v.to_le_bytes())
            .map_err(|_| Error::ToDo)
    }

    // Not particularly efficient but this is example code anyway. A more
    // performant approach would be to use the `itoa` crate.
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output
            .extend_from_slice(&v.to_le_bytes())
            .map_err(|_| Error::ToDo)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output.push(v).map_err(|_| Error::ToDo)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.output
            .extend_from_slice(&v.to_le_bytes())
            .map_err(|_| Error::ToDo)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.output
            .extend_from_slice(&v.to_le_bytes())
            .map_err(|_| Error::ToDo)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output
            .extend_from_slice(&v.to_le_bytes())
            .map_err(|_| Error::ToDo)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        let mut buf = [0u8; core::mem::size_of::<f32>()];
        LittleEndian::write_f32(&mut buf, v);
        self.output
            .extend_from_slice(&buf)
            .map_err(|_| Error::ToDo)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        let mut buf = [0u8; core::mem::size_of::<f64>()];
        LittleEndian::write_f64(&mut buf, v);
        self.output
            .extend_from_slice(&buf)
            .map_err(|_| Error::ToDo)
    }

    // Serialize a char as a single-character string. Other formats may
    // represent this differently.
    fn serialize_char(self, v: char) -> Result<()> {
        let mut buf = [0u8; 4];
        let strsl = v.encode_utf8(&mut buf);
        strsl.serialize(&mut *self)
    }

    // This only works for strings that don't require escape sequences but you
    // get the idea. For example it would emit invalid JSON if the input string
    // contains a '"' character.
    fn serialize_str(self, v: &str) -> Result<()> {
        self.push_usize(v.len())?;
        self.output.extend_from_slice(v.as_bytes()).map_err(|_| Error::ToDo)?;
        Ok(())
    }

    // Serialize a byte array as an array of bytes. Could also use a base64
    // string here. Binary formats will typically represent byte arrays more
    // compactly.
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output.extend_from_slice(v).map_err(|_| Error::ToDo)
    }

    // An absent optional is represented as the JSON `null`.
    fn serialize_none(self) -> Result<()> {
        self.serialize_u8(0)
    }

    // A present optional is represented as just the contained value. Note that
    // this is a lossy representation. For example the values `Some(())` and
    // `None` both serialize as just `null`. Unfortunately this is typically
    // what people expect when working with JSON. Other formats are encouraged
    // to behave more intelligently if possible.
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_u8(1)?;
        value.serialize(&mut *self)
    }

    // In Serde, unit means an anonymous value containing no data. Map this to
    // JSON as `null`.
    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    // Unit struct means a named value containing no data. Again, since there is
    // no data, map this to JSON as `null`. There is no need to serialize the
    // name in most formats.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Ok(())
    }

    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.push_usize(variant_index as usize)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.push_usize(variant_index as usize)?;
        value.serialize(&mut *self)
    }

    // Now we get to the serialization of compound types.
    //
    // The start of the sequence, each value, and the end are three separate
    // method calls. This one is responsible only for serializing the start,
    // which in JSON is `[`.
    //
    // The length of the sequence may or may not be known ahead of time. This
    // doesn't make a difference in JSON because the length is not represented
    // explicitly in the serialized form. Some serializers may only be able to
    // support sequences for which the length is known up front.
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.push_usize(len.ok_or(Error::ToDo)?)?;
        Ok(Self::SerializeSeq{ de: self })
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(Self::SerializeTuple{ de: self })
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(SerializeTupleStruct{ de: self })
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.push_usize(variant_index as usize)?;
        Ok(SerializeTupleVariant{ de: self })
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        // self.output += "{";
        // Ok(self)
        unimplemented!()
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        Ok(Self::SerializeStruct{ de: self })
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.push_usize(variant_index as usize)?;
        Ok(Self::SerializeStructVariant { de: self })
    }

    fn collect_str<T: ?Sized>(self, _value: &T) -> Result<Self::Ok>
    where
        T: core::fmt::Display,
    {
        unreachable!()
    }
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a, B> ser::SerializeSeq for SerializeSeq<'a, B>
where
    B: heapless::ArrayLength<u8>
{
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.de)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuples.
impl<'a, B> ser::SerializeTuple for SerializeTuple<'a, B>
where
    B: heapless::ArrayLength<u8>
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.de)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a, B> ser::SerializeTupleStruct for SerializeTupleStruct<'a, B>
where
    B: heapless::ArrayLength<u8>
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.de)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a, B> ser::SerializeTupleVariant for SerializeTupleVariant<'a, B>
where
    B: heapless::ArrayLength<u8>
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.de)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a, B> ser::SerializeMap for &'a mut Serializer<B>
where
    B: heapless::ArrayLength<u8>
{
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // if !self.output.ends_with('{') {
        //     self.output += ",";
        // }
        // key.serialize(&mut **self)
        unimplemented!()
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // self.output += ":";
        // value.serialize(&mut **self)
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        // self.output += "}";
        // Ok(())
        unimplemented!()
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a, B> ser::SerializeStruct for SerializeStruct<'a, B>
where
    B: heapless::ArrayLength<u8>
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.de)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a, B> ser::SerializeStructVariant for SerializeStructVariant<'a, B>
where
    B: heapless::ArrayLength<u8>
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.de)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
    use heapless::consts::*;
    use super::*;
    use core::ops::Deref;
    use heapless::String;
    use core::fmt::Write;

    #[test]
    fn ser_u8() {
        let output: Vec<u8, U1> = to_vec(&0x05u8).unwrap();
        assert!(&[5] == output.deref());
    }

    #[test]
    fn ser_u16() {
        let output: Vec<u8, U2> = to_vec(&0xA5C7u16).unwrap();
        assert!(&[0xC7, 0xA5] == output.deref());
    }

    #[test]
    fn ser_u32() {
        let output: Vec<u8, U4> = to_vec(&0xCDAB3412u32).unwrap();
        assert!(&[0x12, 0x34, 0xAB, 0xCD] == output.deref());
    }

    #[test]
    fn ser_u64() {
        let output: Vec<u8, U8> = to_vec(&0x1234_5678_90AB_CDEFu64).unwrap();
        assert!(&[0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12] == output.deref());
    }

    #[derive(Serialize)]
    struct BasicU8S {
        st: u16,
        ei: u8,
        sf: u64,
        tt: u32,
    }

    #[test]
    fn ser_struct_unsigned() {
        let output: Vec<u8, U15> = to_vec(
            &BasicU8S {
                st: 0xABCD,
                ei: 0xFE,
                sf: 0x1234_4321_ABCD_DCBA,
                tt: 0xACAC_ACAC
            }).unwrap();

        assert!(&[
            0xCD, 0xAB,
            0xFE,
            0xBA, 0xDC, 0xCD, 0xAB, 0x21, 0x43, 0x34, 0x12,
            0xAC, 0xAC, 0xAC, 0xAC
        ] == output.deref());
    }

    #[test]
    fn ser_byte_slice() {
        let input: &[u8] = &[1u8, 2, 3, 4, 5, 6, 7, 8];
        let output: Vec<u8, U9> = to_vec(input).unwrap();
        assert_eq!(&[
            0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08
        ], output.deref());

        let mut input: Vec<u8, U1024> = Vec::new();
        for i in 0..1024 {
            input.push((i & 0xFF) as u8).unwrap();
        }
        let output: Vec<u8, U2048> = to_vec(input.deref()).unwrap();
        assert_eq!(&[0x80, 0x08], &output.deref()[..2]);

        assert_eq!(output.len(), 1026);
        for (i, val) in output.deref()[2..].iter().enumerate() {
            assert_eq!((i & 0xFF) as u8, *val);
        }
    }

    #[test]
    fn ser_str() {
        let input: &str = "hello, postcard!";
        let output: Vec<u8, U128> = to_vec(input).unwrap();
        assert_eq!(0x10, output.deref()[0]);
        assert_eq!(input.as_bytes(), &output.deref()[1..]);

        let mut input: String<U1024> = String::new();
        for _ in 0..256 {
            write!(&mut input, "abcd").unwrap();
        }
        let output: Vec<u8, U2048> = to_vec(input.deref()).unwrap();
        assert_eq!(&[0x80, 0x08], &output.deref()[..2]);

        assert_eq!(output.len(), 1026);
        for ch in output.deref()[2..].chunks(4) {
            assert_eq!("abcd", core::str::from_utf8(ch).unwrap());
        }
    }

    #[test]
    fn usize_varint_encode() {
        let mut buf = new_varint_buf();
        let res = usize_to_varint(
            1usize,
            &mut buf,
        );

        assert!(&[1] == res);

        let res = usize_to_varint(
            usize::max_value(),
            // 0xFFFFFFFF,
            &mut buf
        );

        // AJM TODO
        if VARINT_MAX_SZ == 5 {
            assert_eq!(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F], res);
        } else {
            assert_eq!(&[0xFF, 0xFF, 0xFF, 0xFF,
                         0xFF, 0xFF, 0xFF, 0xFF,
                         0xFF, 0x01], res);
        }
    }

    #[allow(dead_code)]
    #[derive(Serialize)]
    enum BasicEnum {
        Bib,
        Bim,
        Bap
    }

    #[derive(Serialize)]
    struct EnumStruct {
        eight: u8,
        sixt: u16,
    }

    #[derive(Serialize)]
    enum DataEnum {
        Bib(u16),
        Bim(u64),
        Bap(u8),
        Kim(EnumStruct),
        Chi{ a: u8, b: u32 },
        Sho(u16, u8),
    }

    #[test]
    fn enums() {
        let output: Vec<u8, U1> = to_vec(&BasicEnum::Bim).unwrap();
        assert_eq!(&[0x01], output.deref());

        let output: Vec<u8, U9> = to_vec(&DataEnum::Bim(u64::max_value())).unwrap();
        assert_eq!(&[
            0x01,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
        ], output.deref());

        let output: Vec<u8, U3> = to_vec(&DataEnum::Bib(u16::max_value())).unwrap();
        assert_eq!(&[
            0x00,
            0xFF, 0xFF
        ], output.deref());

        let output: Vec<u8, U2> = to_vec(&DataEnum::Bap(u8::max_value())).unwrap();
        assert_eq!(&[
            0x02,
            0xFF
        ], output.deref());

        let output: Vec<u8, U8> = to_vec(&DataEnum::Kim(EnumStruct { eight: 0xF0, sixt: 0xACAC })).unwrap();
        assert_eq!(&[
            0x03,
            0xF0,
            0xAC, 0xAC,
        ], output.deref());

        let output: Vec<u8, U8> = to_vec(&DataEnum::Chi{ a: 0x0F, b: 0xC7C7C7C7 }).unwrap();
        assert_eq!(&[
            0x04,
            0x0F,
            0xC7, 0xC7, 0xC7, 0xC7
        ], output.deref());

        let output: Vec<u8, U8> = to_vec(&DataEnum::Sho(0x6969, 0x07)).unwrap();
        assert_eq!(&[
            0x05,
            0x69, 0x69,
            0x07
        ], output.deref());
    }

    #[test]
    fn tuples() {
        let output: Vec<u8, U128> = to_vec(&(1u8, 10u32, "Hello!")).unwrap();
        assert_eq!(&[
            1u8,
            0x0A, 0x00, 0x00, 0x00,
            0x06,
            b'H', b'e', b'l', b'l', b'o', b'!'], output.deref())
    }

    #[test]
    fn bytes() {
        let x: &[u8; 32] = &[0u8; 32];
        let output: Vec<u8, U128> = to_vec(x).unwrap();
        assert_eq!(output.len(), 32);
    }

    #[derive(Serialize)]
    pub struct NewTypeStruct(u32);

    #[derive(Serialize)]
    pub struct TupleStruct((u8, u16));

    #[test]
    fn structs() {
        let output: Vec<u8, U4> = to_vec(&NewTypeStruct(5)).unwrap();
        assert_eq!(&[0x05, 0x00, 0x00, 0x00], output.deref());

        let output: Vec<u8, U3> = to_vec(&TupleStruct((0xA0, 0x1234))).unwrap();
        assert_eq!(&[0xA0, 0x34, 0x12], output.deref());
    }

    #[test]
    fn unit() {
        let output: Vec<u8, U1> = to_vec(&()).unwrap();
        assert_eq!(output.len(), 0);
    }
}