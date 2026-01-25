use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

use crate::enums::Language;

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

        $( #[$meta:meta] )*

        $access:vis enum $name:ident {
            $( $arm:ident = $val:literal ),* $(,)?
        }
    ) => {
        #[derive(Debug, Copy, Clone)]
        $( #[$meta] )*
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

pub fn read_string<R: Read + Seek>(
    reader: &mut R,
    lang: Language,
) -> Result<String, Box<dyn Error>> {
    match lang {
        Language::English => read_string_en(reader),
        Language::Japanese => read_string_ja(reader),
        Language::French => read_string_fr(reader),
        Language::German => read_string_de(reader),
        Language::Unknown(_) => unreachable!(),
    }
}

// depending on the outcome, maybe merging of at least EN/FR/DE is possible?
pub fn read_string_en<R: Read + Seek>(reader: &mut R) -> Result<String, Box<dyn Error>> {
    let len = reader.read_u8()?;
    if len == 1 {
        assert_eq!(reader.read_u8()?, 0);
        return Ok("".into());
    }

    let mut buffer = vec![0u8; len as usize - 1];
    reader.read(&mut buffer)?;
    assert_eq!(reader.read_u8()?, 0);
    let s = str::from_utf8(&buffer)?;
    Ok(s.into())
}

pub fn read_string_ja<R: Read + Seek>(reader: &mut R) -> Result<String, Box<dyn Error>> {
    let len = reader.read_u8()?;
    if len == 1 {
        assert_eq!(reader.read_u8()?, 0);
        return Ok("".into());
    }

    let mut buffer = vec![0u8; len as usize - 1];
    reader.read(&mut buffer)?;
    assert_eq!(reader.read_u8()?, 0);

    if buffer[0] == 0xFD && buffer[1] == 0x6A {
        let s = buffer
            .iter()
            .skip(2)
            .map(|b| {
                let c = JAPANESE_CHARACTERS[*b as usize];
                if c == '_' {
                    // panic!("{b:X} is unknown.");
                }
                c
            })
            .collect::<String>();
        println!("{s}");
        Ok(s.into())
    } else {
        let s = str::from_utf8(&buffer)?;
        Ok(s.into())
    }
}

pub fn read_string_fr<R: Read + Seek>(reader: &mut R) -> Result<String, Box<dyn Error>> {
    let len = reader.read_u8()?;
    if len == 1 {
        assert_eq!(reader.read_u8()?, 0);
        return Ok("".into());
    }

    let mut buffer = vec![0u8; len as usize - 1];
    reader.read(&mut buffer)?;
    assert_eq!(reader.read_u8()?, 0);
    let s = buffer
        .iter()
        .map(|c| match c {
            b'a' => todo!(),
            b'b' => 'É',
            b'c' => 'È',
            b'd' => 'Ê',
            b'e'..=b'z' => panic!("FR char is {}", *c as char),
            _ => *c as char,
        })
        .collect::<String>();
    Ok(s.into())
}

pub fn read_string_de<R: Read + Seek>(reader: &mut R) -> Result<String, Box<dyn Error>> {
    let len = reader.read_u8()?;
    if len == 1 {
        assert_eq!(reader.read_u8()?, 0);
        return Ok("".into());
    }

    let mut buffer = vec![0u8; len as usize - 1];
    reader.read(&mut buffer)?;
    assert_eq!(reader.read_u8()?, 0);
    let s = buffer
        .iter()
        .map(|c| match c {
            b']' => 'Ü',
            b'[' => 'Ä',
            b'a'..=b'z' => panic!("DE char is {}", *c as char),
            _ => *c as char,
        })
        .collect::<String>();
    Ok(s.into())
}

#[rustfmt::skip]
const JAPANESE_CHARACTERS: [char; 256] = [
/* 00 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '＆', '_', '_',
/* 10 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', 'A', 'B', 'C', 'D', 'E', 'F',
/* 20 */ 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
/* 30 */ 'W', 'X', 'Y', 'Z', '_', '_', '_', '_', '_', '_', '_', 'ー', '_', '_', '_', '_',
/* 40 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', 'ョ', '_', 'ン',
/* 50 */ '_', '_', '_', '_', 'オ', '_', 'キ', '_', '_', '_', 'サ', 'シ', 'ス', 'セ', 'ソ', 'タ',
/* 60 */ 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', 'マ', 'ミ',
/* 70 */ 'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', '_', '_', '_', 'グ', 'ゲ',
/* 80 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', 'ボ',
/* 90 */ '_', '_', 'プ', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
/* A0 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
/* B0 */ '_', '_', '_', '_', '_', '_', '_', '_', 'を', '_', 'あ', 'い', 'う', 'え', 'お', 'か',
/* C0 */ 'き', 'く', 'け', 'こ', 'さ', 'し', 'す', 'せ', 'そ', 'た', 'ち', 'つ', 'て', 'と', 'な', 'に',
/* D0 */ 'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'ま', 'み', 'む', 'め', 'も', '_', '_', '_',
/* E0 */ '_', '_', 'る', '_', '_', '_', '_', '_', '_', '_', '_', '_', 'じ', '_', '_', '_',
/* F0 */ 'だ', '_', '_', '_', '_', '_', 'び', '_', '_', '_', '_', '_', '_', '_', '_', '_',
];
