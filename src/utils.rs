use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::error::Error;
use std::io;
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

pub fn read_2_floats<R: Read>(reader: &mut R) -> Result<Vector2<f32>, Box<dyn Error>> {
    let x = reader.read_f32::<BigEndian>()?;
    let y = reader.read_f32::<BigEndian>()?;
    Ok(Vector2 { x, y })
}

pub fn read_3_floats<R: Read>(reader: &mut R) -> Result<Vector3<f32>, Box<dyn Error>> {
    let x = reader.read_f32::<BigEndian>()?;
    let y = reader.read_f32::<BigEndian>()?;
    let z = reader.read_f32::<BigEndian>()?;
    Ok(Vector3 { x, y, z })
}

pub fn read_3_u32<R: Read>(reader: &mut R) -> Result<Vector3<u32>, Box<dyn Error>> {
    let x = reader.read_u32::<BigEndian>()?;
    let y = reader.read_u32::<BigEndian>()?;
    let z = reader.read_u32::<BigEndian>()?;
    Ok(Vector3 { x, y, z })
}

pub fn read_3_i16<R: Read>(reader: &mut R) -> Result<Vector3<i16>, Box<dyn Error>> {
    let x = reader.read_i16::<BigEndian>()?;
    let y = reader.read_i16::<BigEndian>()?;
    let z = reader.read_i16::<BigEndian>()?;
    Ok(Vector3 { x, y, z })
}

pub fn read_3_u16<R: Read>(reader: &mut R) -> Result<Vector3<u16>, Box<dyn Error>> {
    let x = reader.read_u16::<BigEndian>()?;
    let y = reader.read_u16::<BigEndian>()?;
    let z = reader.read_u16::<BigEndian>()?;
    Ok(Vector3 { x, y, z })
}

pub fn read_2_i16<R: Read>(reader: &mut R) -> Result<Vector2<i16>, Box<dyn Error>> {
    let x = reader.read_i16::<BigEndian>()?;
    let y = reader.read_i16::<BigEndian>()?;
    Ok(Vector2 { x, y })
}

pub fn read_3_u8<R: Read>(reader: &mut R) -> Result<Vector3<u8>, Box<dyn Error>> {
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

pub fn fixed_to_float(fixed: u16) -> f32 {
    let mut float = (fixed >> 6) as f32;
    let mut decimal = fixed & 0b111111;

    for i in 1..7 {
        let bit = decimal & 0b100000;
        if bit != 0 {
            float += 2f32.powi(-i);
        }

        decimal <<= 1;
    }

    let other_way = float_to_fixed(float);
    assert_eq!(fixed, other_way);

    float
}

pub fn float_to_fixed(float: f32) -> u16 {
    let mut fixed = (float as u16) << 6;
    let mut fract = float.fract();

    for i in 1..7 {
        let part = 2f32.powi(-i);
        if fract >= part {
            fixed += 1 << (6 - i);
            fract -= part;
        }
    }

    fixed
}

pub fn align_reader<R: Read + Seek>(reader: &mut R) -> Result<(), Box<dyn Error>> {
    let pos = reader.seek(std::io::SeekFrom::Current(0))?;
    match pos % 8 {
        0 => {}
        4 => {
            reader.read_u32::<BigEndian>()?;
        }
        _ => {
            println!("align_reader: {pos:X} -> {}", pos % 8);
            unreachable!();
        }
    }

    Ok(())
}

pub fn align_writer<W: Write + Seek>(writer: &mut W) -> Result<(), Box<dyn Error>> {
    let pos = writer.seek(std::io::SeekFrom::Current(0))?;
    let modulo = (pos % 8) as usize;
    if modulo != 0 {
        let mut buffer = vec![0; 8 - modulo];
        io::repeat(0xCD).read_exact(&mut buffer).unwrap();
        writer.write(&buffer)?;
    }

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
        #[derive(Debug, Copy, Clone, PartialEq)]
        $( #[$meta] )*
        $access enum $name {
            $($arm,)*
            Unknown($typ),
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$arm => write!(f, "{}", stringify!($arm)) ,)*
                    $name::Unknown(v) => write!(f, "Unknown({v}, {v:X})"),
                }
            }
        }

        impl From<$typ> for $name {
            fn from(value: $typ) -> Self {
                #[allow(unreachable_patterns)]
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

pub fn read_string<R: Read>(reader: &mut R) -> Result<String, Box<dyn Error>> {
    let len = reader.read_u8()?;
    if len == 1 {
        assert_eq!(reader.read_u8()?, 0);
        return Ok("".into());
    }

    let mut buffer = vec![0u8; len as usize - 1];
    reader.read(&mut buffer)?;
    assert_eq!(reader.read_u8()?, 0);

    let s = convert_from_banjo_string(buffer);
    Ok(s)
}

pub fn write_string<W: Write>(writer: &mut W, string: &str) -> Result<(), Box<dyn Error>> {
    let buffer = convert_to_banjo_string(string);
    writer.write_u8(1 + buffer.len() as u8)?;
    if buffer.len() > 0 {
        writer.write(&buffer)?;
    }
    writer.write_u8(0)?;

    Ok(())
}

pub fn convert_to_banjo_string(string: &str) -> Vec<u8> {
    let mut buffer = vec![0xFD, 0x6A];
    let mut mapping = JAPANESE_CHARACTERS;

    if string.chars().all(|c| match c {
        '⸾' | '⸽' => true,
        _ => CHARACTERS.contains(&c),
    }) {
        mapping = CHARACTERS;
        buffer.clear();
    }

    for c in string.chars() {
        if c == '⸾' {
            buffer.push(0xFD);
            buffer.push(0x68);
        } else if c == '⸽' {
            buffer.push(0xFD);
            buffer.push(0x6C);
        } else {
            if let Some(index) = mapping.iter().position(|k| *k == c) {
                buffer.push(index as u8);
            } else {
                unreachable!();
            }
        }
    }

    buffer
}

pub fn convert_from_banjo_string(buffer: Vec<u8>) -> String {
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
                0x68 => {
                    /*wiggle start?*/
                    target_buffer.push('⸾');
                }
                0x6A => japanese = true,
                0x6C => {
                    /*wiggle stop?*/
                    target_buffer.push('⸽');
                }
                _ => panic!("Unknown control character {code:X}"),
            }
        } else if japanese {
            let target = JAPANESE_CHARACTERS[c as usize];
            target_buffer.push(target);
        } else {
            let target = CHARACTERS[c as usize];
            assert_ne!(target, '_');
            target_buffer.push(target);
        }
    }

    target_buffer.iter().collect::<String>()
}

pub fn convert_to_iso_8859_1(string: &String) -> Vec<u8> {
    string
        .chars()
        .map(|b| match b {
            '\n' => b'\n',
            ' '..='~' => b as u8,
            '\u{00a0}' => b as u8, // NBSP
            'É' => 0xC9,
            'Ü' => 0xDC,
            'ß' => 0xDF,
            'à' => 0xE0,
            'ä' => 0xE4,
            'è' => 0xE8,
            'é' => 0xE9,
            'ê' => 0xEA,
            'ö' => 0xF6,
            'ü' => 0xFC,
            _ => panic!("unhandled char '{b}'"),
        })
        .collect::<Vec<_>>()
}

pub fn convert_from_iso_8859_1(buffer: Vec<u8>) -> String {
    buffer
        .iter()
        .map(|b| match b {
            0x0A => '\n',
            0x20..=0x7E => *b as char,
            0xA0 => *b as char, // NBSP
            0xC9 => 'É',
            0xDC => 'Ü',
            0xDF => 'ß',
            0xE0 => 'à',
            0xE4 => 'ä',
            0xE8 => 'è',
            0xE9 => 'é',
            0xEA => 'ê',
            0xF6 => 'ö',
            0xFC => 'ü',
            _ => panic!("unhandled byte {b:X}"),
        })
        .collect::<String>()
}

#[rustfmt::skip]
const CHARACTERS: [char; 256] = [
/* 00 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
/* 10 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
/* 20 */ ' ', '!', '_', '_', '_', '_', '&', '\'', '(', ')', '_', '+', ',', '-', '.', '/',
/* 30 */ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '©', '_', '_', '?',
/* 40 */ '_', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
/* 50 */ 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'Ä', 'Ö', 'Ü', 'ß', 'À', // ß is random for now
/* 60 */ 'Â', 'Ç', 'É', 'È', 'Ê', 'Ë', 'Î', 'Ï', 'Ô', 'Û', '_', 'Ù', '_', '_', '_', '_',
/* 70 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '~', '_',
/* 80 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
/* 90 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
/* A0 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
/* B0 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
/* C0 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
/* D0 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
/* E0 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
/* F0 */ '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_',
];

#[rustfmt::skip]
const JAPANESE_CHARACTERS: [char; 256] = [
/* 00 */ '_', '、', '$', '(', ')', '.', '%', '「', '」', '_', '_', '<', '>', '＆', '~', ' ',
/* 10 */ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
/* 20 */ 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
/* 30 */ 'W', 'X', 'Y', 'Z', '!', '"', '#', '\'', '*', '+', ',', 'ー', '.', '/', ':', '=',
/* 40 */ '?', '@', '。', '゛', '゜', 'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ッ', 'ャ', 'ュ', 'ョ', 'ヲ', 'ン',
/* 50 */ 'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ', 'タ',
/* 60 */ 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', 'マ', 'ミ',
/* 70 */ 'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ', 'ガ', 'ギ', 'グ', 'ゲ',
/* 80 */ 'ゴ', 'ザ', 'ジ', 'ズ', 'ゼ', 'ゾ', 'ダ', 'ヂ', 'ヅ', 'デ', 'ド', 'バ', 'ビ', 'ブ', 'ベ', 'ボ',
/* 90 */ 'パ', 'ピ', 'プ', 'ペ', 'ポ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
/* A0 */ 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'ぁ',
/* B0 */ 'ぃ', 'ぅ', 'ぇ', 'ぉ', 'っ', 'ゃ', 'ゅ', 'ょ', 'を', 'ん', 'あ', 'い', 'う', 'え', 'お', 'か',
/* C0 */ 'き', 'く', 'け', 'こ', 'さ', 'し', 'す', 'せ', 'そ', 'た', 'ち', 'つ', 'て', 'と', 'な', 'に',
/* D0 */ 'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'ま', 'み', 'む', 'め', 'も', 'や', 'ゆ', 'よ',
/* E0 */ 'ら', 'り', 'る', 'れ', 'ろ', 'わ', 'が', 'ぎ', 'ぐ', 'げ', 'ご', 'ざ', 'じ', 'ず', 'ぜ', 'ぞ',
/* F0 */ 'だ', 'ぢ', 'づ', 'で', 'ど', 'ば', 'び', 'ぶ', 'べ', 'ぼ', 'ぱ', 'ぴ', 'ぷ', 'ぺ', 'ぽ', 'ヴ',
];
