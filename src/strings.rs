use crate::enums::Language;
use crate::utils::convert_from_banjo_string;
use crate::utils::convert_from_iso_8859_1;
use crate::utils::convert_to_banjo_string;
use crate::utils::convert_to_iso_8859_1;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

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
            let language = LANGUAGE_MAPPING[lang];

            for str_id in 0..number_of_strings {
                let len = sizes_per_language[lang][str_id];

                let mut buffer = vec![0u8; len - 1];
                reader.read(&mut buffer)?;
                assert_eq!(reader.read_u8()?, 0);

                let s = match language {
                    Language::English | Language::French | Language::German => {
                        if IS_BANJO_STRING[str_id] {
                            convert_from_banjo_string(buffer)
                        } else {
                            convert_from_iso_8859_1(buffer)
                        }
                    }
                    Language::Japanese => "".into(),
                    Language::Unknown(_) => unreachable!(),
                };

                if lang == 0 {
                    let mut translations = HashMap::new();
                    translations.insert(Language::English, s);
                    strings.push(TranslatableString { translations });
                } else {
                    let data = strings.get_mut(str_id).unwrap();
                    data.translations.insert(LANGUAGE_MAPPING[lang], s);
                }
            }
        }

        Ok(Self { strings })
    }

    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        let number_of_strings = self.strings.len();
        let number_of_languages = self.strings[0].translations.len();
        writer.write_u16::<LittleEndian>(number_of_strings as u16)?;
        writer.write_u16::<LittleEndian>(number_of_languages as u16)?;

        let sizes_pos = writer.seek(std::io::SeekFrom::Current(0))?;
        for _ in 0..number_of_languages {
            writer.write_u32::<LittleEndian>(0)?;
        }

        for lang_id in 0..number_of_languages {
            let lang = LANGUAGE_MAPPING[lang_id];
            for string in &self.strings {
                writer.write_u32::<LittleEndian>(string.translations[&lang].len() as u32 + 1)?;
            }
        }

        let mut sizes_by_languages = vec![];
        for lang_id in 0..number_of_languages {
            let lang = LANGUAGE_MAPPING[lang_id];

            let begin = writer.seek(std::io::SeekFrom::Current(0))?;
            for (str_id, string) in self.strings.iter().enumerate() {
                match lang {
                    Language::Japanese => {
                        // use English until JP encoding has been understood
                        writer.write(string.translations[&Language::English].as_bytes())?;
                    }
                    Language::English | Language::French | Language::German => {
                        let buffer = if IS_BANJO_STRING[str_id] {
                            convert_to_banjo_string(&string.translations[&lang])
                        } else {
                            convert_to_iso_8859_1(&string.translations[&lang])
                        };

                        writer.write(&buffer)?;
                    }
                    Language::Unknown(_) => unreachable!(),
                }
                writer.write_u8(0)?;
            }
            let end = writer.seek(std::io::SeekFrom::Current(0))?;
            sizes_by_languages.push(end - begin);
        }

        writer.seek(std::io::SeekFrom::Start(sizes_pos))?;
        for i in 0..number_of_languages {
            writer.write_u32::<LittleEndian>(sizes_by_languages[i] as u32)?;
        }

        Ok(())
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
