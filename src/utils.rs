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

pub fn read_2_i16<R: Read + Seek>(reader: &mut R) -> Result<Vector2<i16>, Box<dyn Error>> {
    let x = reader.read_i16::<BigEndian>()?;
    let y = reader.read_i16::<BigEndian>()?;
    Ok(Vector2 { x, y })
}

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

pub fn write_2_i16<W: Write>(writer: &mut W, vec: &Vector2<i16>) -> Result<(), Box<dyn Error>> {
    writer.write_i16::<BigEndian>(vec.x)?;
    writer.write_i16::<BigEndian>(vec.y)?;
    Ok(())
}

pub fn write_3_u32<W: Write>(writer: &mut W, vec: &Vector3<u32>) -> Result<(), Box<dyn Error>> {
    writer.write_u32::<BigEndian>(vec.x)?;
    writer.write_u32::<BigEndian>(vec.y)?;
    writer.write_u32::<BigEndian>(vec.z)?;
    Ok(())
}

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

pub fn read_string<R: Read + Seek>(reader: &mut R) -> Result<String, Box<dyn Error>> {
    let len = reader.read_u8()?;
    if len == 1 {
        assert_eq!(reader.read_u8()?, 0);
        return Ok("".into());
    }

    let mut buffer = vec![0u8; len as usize - 1];
    reader.read(&mut buffer)?;
    assert_eq!(reader.read_u8()?, 0);

    let s = convert_banjo_string(buffer);
    Ok(s)
}

pub fn convert_banjo_string(buffer: Vec<u8>) -> String {
    let mut index = 0;
    let mut japanese = false;
    let mut target_buffer = vec![];
    while index < buffer.len() {
        let c = buffer[index];
        index += 1;

        // control character
        if c == 0xFD {
            let code = buffer[index];
            index += 1;

            match code {
                0x68 => { /*wiggle start?*/ }
                0x6A => japanese = true,
                0x6C => { /*wiggle stop?*/ }
                _ => panic!("Unknown control character {code:X}"),
            }
        } else if japanese {
            let target = JAPANESE_CHARACTERS[c as usize];
            if target == '_' {
                // panic!("{b:X} / {b} is unknown.");
            }
            target_buffer.push(target);
        } else {
            let target = match c {
                b'A'..=b'Z' => c as char,
                b'0'..=b'9' => c as char,
                b'a' => 'Ç',
                b'b' => 'É',
                b'c' => 'È',
                b'd' => 'Ê',
                b'e' => 'Ë',
                b'f' => 'Î',
                b'g' => 'Ï',
                b'h' => 'Ô',
                b'i' => 'Û',
                b'k' => 'Ù',
                b'`' => 'Â',
                b'_' => 'À',
                b']' => 'Ü',
                b'\\' => 'Ö', // for dialogues. unknown for X360_strings.dat
                b'[' => 'Ä',
                b'^' => 'ß', // unknown (random letter for now)
                b'\'' => '\'',
                b' ' => ' ',
                b'!' => '!',
                b'.' => '.',
                b',' => ',',
                b'?' => '?',
                b'-' => '-',
                b':' => ':',
                b';' => ';',
                b'&' => '&',
                b'(' => '(',
                b')' => ')',
                b'/' => '/',
                b'+' => '+',
                b'<' => '©',
                b'~' => '~', // placeholder
                _ => panic!("char {c} / {c:X} / {} unknown", c as char),
            };
            target_buffer.push(target);
        }
    }

    target_buffer.iter().collect::<String>()
}

pub fn convert_iso_8859_1(buffer: Vec<u8>) -> String {
    buffer
        .iter()
        .map(|b| match b {
            0x20..=0x7E => *b as char,
            0xA0 => *b as char, // NBSP
            0xC9 => 'É',
            0xE0 => 'à',
            0xE8 => 'è',
            0xE9 => 'é',
            0xEA => 'ê',
            _ => panic!("unhandled byte {b:X}"),
        })
        .collect::<String>()
}

#[rustfmt::skip]
const JAPANESE_CHARACTERS: [char; 256] = [
/* 00 */ '_', '、', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '＆', '_', ' ',
/* 10 */ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
/* 20 */ 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
/* 30 */ 'W', 'X', 'Y', 'Z', '!', '_', '_', '_', '_', '_', '_', 'ー', '_', '_', '_', '_',
/* 40 */ '_', '_', '。', '_', '_', '_', '_', '_', '_', 'ォ', '_', '_', '_', 'ョ', '_', 'ン',
/* 50 */ 'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ', 'タ',
/* 60 */ 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', 'マ', 'ミ',
/* 70 */ 'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', '_', '_', '_', 'グ', 'ゲ',
/* 80 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', 'ボ',
/* 90 */ '_', '_', 'プ', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
/* A0 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
/* B0 */ '_', '_', '_', '_', 'っ', '_', '_', '_', 'を', '_', 'あ', 'い', 'う', 'え', 'お', 'か',
/* C0 */ 'き', 'く', 'け', 'こ', 'さ', 'し', 'す', 'せ', 'そ', 'た', 'ち', 'つ', 'て', 'と', 'な', 'に',
/* D0 */ 'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'ま', 'み', 'む', 'め', 'も', 'や', 'ゆ', 'よ',
/* E0 */ 'ら', 'り', 'る', 'れ', 'ろ', 'わ', 'が', 'ぎ', 'ぐ', 'げ', 'ご', 'ざ', 'じ', 'ず', 'ぜ', 'ぞ',
/* F0 */ 'だ', 'ぢ', 'づ', 'で', 'ど', 'ば', 'び', 'ぶ', 'べ', 'ぼ', '_', '_', '_', '_', '_', '_',
];
