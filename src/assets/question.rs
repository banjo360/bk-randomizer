use crate::utils::read_string;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::error::Error;
use std::io::Read;
use std::io::Seek;

#[derive(Debug)]
pub struct QuestionData {
    question: Vec<String>,
    answer_1: Vec<String>,
    answer_2: Vec<String>,
    answer_3: Vec<String>,
}

pub struct Question {
    // pub questions: HashMap<Language, QuestionData>,
}

impl Question {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let languages = reader.read_u8()?;
        assert_eq!(languages, 4);

        let _unk = reader.read_u16::<LittleEndian>()?;

        for _ in 0..languages {
            // offsets
            reader.read_u16::<LittleEndian>()?;
        }

        for _ in 0..languages {
            QuestionData::new(reader)?;
        }

        Ok(Self {})
    }
}

impl QuestionData {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
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
}
