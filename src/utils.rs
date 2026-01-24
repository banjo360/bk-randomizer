use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

pub fn read_2_floats<R: Read + Seek>(reader: &mut R) -> Result<Vector2<f32>, Box<dyn Error>> {
    let x = reader.read_f32::<BigEndian>()?;
    let y = reader.read_f32::<BigEndian>()?;
    Ok(Vector2 { x, y })
}

pub fn read_3_floats<R: Read + Seek>(reader: &mut R) -> Result<Vector3<f32>, Box<dyn Error>> {
    let x = reader.read_f32::<BigEndian>()?;
    let y = reader.read_f32::<BigEndian>()?;
    let z = reader.read_f32::<BigEndian>()?;
    Ok(Vector3 { x, y, z })
}

pub fn read_3_u32<R: Read + Seek>(reader: &mut R) -> Result<Vector3<u32>, Box<dyn Error>> {
    let x = reader.read_u32::<BigEndian>()?;
    let y = reader.read_u32::<BigEndian>()?;
    let z = reader.read_u32::<BigEndian>()?;
    Ok(Vector3 { x, y, z })
}

pub fn read_3_i16<R: Read + Seek>(reader: &mut R) -> Result<Vector3<i16>, Box<dyn Error>> {
    let x = reader.read_i16::<BigEndian>()?;
    let y = reader.read_i16::<BigEndian>()?;
    let z = reader.read_i16::<BigEndian>()?;
    Ok(Vector3 { x, y, z })
}

pub fn read_3_u16<R: Read + Seek>(reader: &mut R) -> Result<Vector3<u16>, Box<dyn Error>> {
    let x = reader.read_u16::<BigEndian>()?;
    let y = reader.read_u16::<BigEndian>()?;
    let z = reader.read_u16::<BigEndian>()?;
    Ok(Vector3 { x, y, z })
}

#[allow(dead_code)]
pub fn read_2_i16<R: Read + Seek>(reader: &mut R) -> Result<Vector2<i16>, Box<dyn Error>> {
    let x = reader.read_i16::<BigEndian>()?;
    let y = reader.read_i16::<BigEndian>()?;
    Ok(Vector2 { x, y })
}

#[allow(dead_code)]
pub fn read_3_u8<R: Read + Seek>(reader: &mut R) -> Result<Vector3<u8>, Box<dyn Error>> {
    let x = reader.read_u8()?;
    let y = reader.read_u8()?;
    let z = reader.read_u8()?;
    Ok(Vector3 { x, y, z })
}

pub fn write_2_floats<W: Write>(writer: &mut W, vec: &Vector2<f32>) -> Result<(), Box<dyn Error>> {
    writer.write_f32::<BigEndian>(vec.x)?;
    writer.write_f32::<BigEndian>(vec.y)?;
    Ok(())
}

pub fn write_3_floats<W: Write>(writer: &mut W, vec: &Vector3<f32>) -> Result<(), Box<dyn Error>> {
    writer.write_f32::<BigEndian>(vec.x)?;
    writer.write_f32::<BigEndian>(vec.y)?;
    writer.write_f32::<BigEndian>(vec.z)?;
    Ok(())
}

pub fn write_3_i16<W: Write>(writer: &mut W, vec: &Vector3<i16>) -> Result<(), Box<dyn Error>> {
    writer.write_i16::<BigEndian>(vec.x)?;
    writer.write_i16::<BigEndian>(vec.y)?;
    writer.write_i16::<BigEndian>(vec.z)?;
    Ok(())
}

pub fn write_3_u16<W: Write>(writer: &mut W, vec: &Vector3<u16>) -> Result<(), Box<dyn Error>> {
    writer.write_u16::<BigEndian>(vec.x)?;
    writer.write_u16::<BigEndian>(vec.y)?;
    writer.write_u16::<BigEndian>(vec.z)?;
    Ok(())
}

#[allow(dead_code)]
pub fn write_2_i16<W: Write>(writer: &mut W, vec: &Vector2<i16>) -> Result<(), Box<dyn Error>> {
    writer.write_i16::<BigEndian>(vec.x)?;
    writer.write_i16::<BigEndian>(vec.y)?;
    Ok(())
}

#[allow(dead_code)]
pub fn write_3_u32<W: Write>(writer: &mut W, vec: &Vector3<u32>) -> Result<(), Box<dyn Error>> {
    writer.write_u32::<BigEndian>(vec.x)?;
    writer.write_u32::<BigEndian>(vec.y)?;
    writer.write_u32::<BigEndian>(vec.z)?;
    Ok(())
}

#[allow(dead_code)]
pub fn write_3_u8<W: Write>(writer: &mut W, vec: &Vector3<u8>) -> Result<(), Box<dyn Error>> {
    writer.write_u8(vec.x)?;
    writer.write_u8(vec.y)?;
    writer.write_u8(vec.z)?;
    Ok(())
}

#[macro_export]
macro_rules! enum_builder {
    (
        #[repr($typ:ty)]

        $access:vis enum $name:ident {
            $( $arm:ident = $val:literal ),* $(,)?
        }
    ) => {
        #[derive(Debug, Copy, Clone)]
        $access enum $name {
            $($arm,)*
            Unknown($typ),
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$arm => write!(f, "{}", stringify!($arm)) ,)*
                    $name::Unknown(v) => write!(f, "Unknown({v})"),
                }
            }
        }

        impl From<$typ> for $name {
            fn from(value: $typ) -> Self {
                match value {
                    $($val => $name::$arm,)*
                    _ => $name::Unknown(value),
                }
            }
        }

        impl Into<$typ> for $name {
            fn into(self) -> $typ {
                match self {
                    $($name::$arm => $val ,)*
                    $name::Unknown(v) => v,
                }
            }
        }
    };
}
