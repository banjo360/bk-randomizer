use crate::Language;
use crate::utils::read_string;
use crate::utils::write_string;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

#[derive(Debug)]
pub struct QuestionData {
    question: Vec<String>,
    answer_1: Vec<String>,
    answer_2: Vec<String>,
    answer_3: Vec<String>,
}

pub struct Question {
    pub kind: u16,
    pub translations: HashMap<Language, QuestionData>,
}

impl Question {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let languages = reader.read_u8()?;
        assert_eq!(languages, 4);

        let kind = reader.read_u16::<BigEndian>()?;
        // 0201 = Question
        // 0003 = Grunty Quiz

        // offsets
        for _ in 0..languages {
            reader.read_u16::<BigEndian>()?;
        }

        let mut translations = HashMap::new();
        for lang in 0..languages {
            translations.insert(lang.into(), QuestionData::new(reader)?);
        }

        Ok(Self { kind, translations })
    }

    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        writer.write_u8(self.translations.len() as u8)?;
        writer.write_u16::<BigEndian>(self.kind)?;

        let offsets_position = writer.seek(SeekFrom::Current(0))?;

        for _ in 0..self.translations.len() {
            writer.write_u16::<BigEndian>(0)?;
        }

        let mut offsets = vec![];
        let header_offset = writer.seek(SeekFrom::Current(0))?;
        for lang in 0..(self.translations.len() as u8) {
            let offset = (writer.seek(SeekFrom::Current(0))? - header_offset) as u16;
            offsets.push(offset);
            self.translations[&lang.into()].write(writer)?;
        }

        let end_of_file = writer.seek(SeekFrom::Current(0))?;

        writer.seek(SeekFrom::Start(offsets_position))?;
        for offset in offsets {
            writer.write_u16::<BigEndian>(offset)?;
        }

        writer.seek(SeekFrom::Start(end_of_file))?;

        Ok(())
    }
}

impl QuestionData {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let count = reader.read_u8()?;

        let mut question = vec![];
        let mut answer_1 = vec![];
        let mut answer_2 = vec![];
        let mut answer_3 = vec![];

        for _ in 0..count {
            let command_id = reader.read_u8()?;
            match command_id {
                0x80 => question.push(read_string(reader)?),
                0x81 => answer_1.push(read_string(reader)?),
                0x82 => answer_2.push(read_string(reader)?),
                0x83 => answer_3.push(read_string(reader)?),
                _ => unreachable!(),
            }
        }

        Ok(Self {
            question,
            answer_1,
            answer_2,
            answer_3,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        let count =
            self.question.len() + self.answer_1.len() + self.answer_2.len() + self.answer_3.len();
        writer.write_u8(count as u8)?;

        for q in &self.question {
            writer.write_u8(0x80)?;
            write_string(writer, &q)?;
        }

        for a in &self.answer_1 {
            writer.write_u8(0x81)?;
            write_string(writer, &a)?;
        }

        for a in &self.answer_2 {
            writer.write_u8(0x82)?;
            write_string(writer, &a)?;
        }

        for a in &self.answer_3 {
            writer.write_u8(0x83)?;
            write_string(writer, &a)?;
        }

        Ok(())
    }
}
