use crate::enums::Language;
use crate::utils::convert_banjo_string;
use crate::utils::convert_iso_8859_1;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::io::Seek;

enum StringId {}

pub struct TranslatableString {
    pub translations: HashMap<Language, String>,
}
pub struct Strings {
    pub strings: Vec<TranslatableString>,
}

const LANGUAGE_MAPPING: [Language; 4] = [
    Language::English,
    Language::French,
    Language::German,
    Language::Japanese,
];

impl Strings {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let number_of_strings = reader.read_u16::<LittleEndian>()? as usize;
        let number_of_languages = reader.read_u16::<LittleEndian>()? as usize;

        // sizes of languages
        for _ in 0..number_of_languages {
            reader.read_u32::<LittleEndian>()?;
        }

        let mut sizes_per_language = vec![];
        for _ in 0..number_of_languages {
            let mut sizes = vec![];
            for _ in 0..number_of_strings {
                sizes.push(reader.read_u32::<LittleEndian>()? as usize);
            }
            sizes_per_language.push(sizes);
        }

        let mut strings = vec![];
        for lang in 0..number_of_languages {
            for str_id in 0..number_of_strings {
                let len = sizes_per_language[lang][str_id];
                let pos = reader.seek(std::io::SeekFrom::Current(0))?;
                let mut buffer = vec![0u8; len - 1];
                reader.read(&mut buffer)?;
                assert_eq!(reader.read_u8()?, 0);

                let s = match lang {
                    0 => String::from_utf8(buffer)?,
                    1 => {
                        if IS_BANJO_STRING[str_id] {
                            convert_banjo_string(buffer)
                        } else {
                            convert_iso_8859_1(buffer)
                        }
                    }
                    _ => todo!(),
                };

                if lang == 0 {
                    let mut translations = HashMap::new();
                    translations.insert(Language::English, s);
                    strings.push(TranslatableString { translations });
                } else {
                    let mut data = strings.get_mut(str_id).unwrap();
                    data.translations.insert(LANGUAGE_MAPPING[lang], s);
                }
            }
        }

        let pos = reader.seek(std::io::SeekFrom::Current(0))?;
        println!("{pos:X}");

        todo!();

        Ok(Self { strings })
    }
}

#[rustfmt::skip]
const IS_BANJO_STRING: [bool; 169] = [
true,  true,  true,  true,  true,  true,  true,  true,  true,  true,
true,  true,  true,  true,  true,  true,  true,  true,  true,  true,
true,  true,  true,  true,  true,  true,  true,  true,  true,  true,
true,  true,  true,  false, false, false, false, false, false, false,
false, true,  false, true,  false, false, true,  true,  true,  true,
true,  true,  true,  true,  true,  true,  true,  true,  false, true,
true,  true,  true,  true,  true,  true,  true,  true,  true,  true,
true,  true,  true,  true,  true,  true,  true,  true,  true,  true,
true,  true,  true,  true,  true,  true,  true,  true,  true,  true,
true,  true,  true,  true,  true,  true,  true,  true,  true,  true,
true,  true,  true,  true,  true,  true,  true,  true,  true,  true,
true,  true,  true,  true,  false, false, false, false, true,  true,
true,  false, false, false, false, true,  false, false, false, false,
true,  true,  false, false, false, true,  false, false, false, true,
false, false, false, false, false, false, false, false, false, true,
true,  true,  true,  true,  true,  true,  true,  false, false, false,
false, false, false, false, false, false, true,  true,  false,
];
