use std::io::{Read, Write};

use crate::errors::Result;
use crate::ngram_model::NgramModel;
use crate::utils;

pub struct TagClassInfo {
    pub(crate) name: String,
    pub(crate) bias: i32,
}

impl TagClassInfo {
    pub fn serialize<W>(&self, mut wtr: W) -> Result<()>
    where
        W: Write,
    {
        utils::write_u32(&mut wtr, self.name.len().try_into().unwrap())?;
        wtr.write_all(self.name.as_bytes())?;
        utils::write_i32(&mut wtr, self.bias)?;
        Ok(())
    }

    pub fn deserialize<R>(mut rdr: R) -> Result<Self>
    where
        R: Read,
    {
        let name_size = utils::read_u32(&mut rdr)?;
        let mut name_bytes = vec![0; name_size.try_into().unwrap()];
        rdr.read_exact(&mut name_bytes)?;
        let name = String::from_utf8(name_bytes)?;
        Ok(Self {
            name,
            bias: utils::read_i32(&mut rdr)?,
        })
    }
}

// Left and right weight arrays of the TagModel are ordered as follows:
//
//      tok1 tok2 tok3 ...
//
// tag1   1    5    9
// tag2   2    6    .
// tag3   3    7    .
// ...    4    8    .
#[derive(Default)]
pub struct TagModel {
    pub(crate) class_info: Vec<TagClassInfo>,
    pub(crate) left_char_model: NgramModel<String>,
    pub(crate) right_char_model: NgramModel<String>,
    pub(crate) self_char_model: NgramModel<String>,
}

impl TagModel {
    pub fn serialize<W>(&self, mut wtr: W) -> Result<()>
    where
        W: Write,
    {
        utils::write_u32(&mut wtr, self.class_info.len().try_into().unwrap())?;
        for cls in &self.class_info {
            cls.serialize(&mut wtr)?;
        }
        self.left_char_model.serialize(&mut wtr)?;
        self.right_char_model.serialize(&mut wtr)?;
        self.self_char_model.serialize(&mut wtr)?;
        Ok(())
    }

    pub fn deserialize<R>(mut rdr: R) -> Result<Self>
    where
        R: Read,
    {
        let n_class = utils::read_u32(&mut rdr)?;
        let mut class_info = vec![];
        for _ in 0..n_class {
            class_info.push(TagClassInfo::deserialize(&mut rdr)?);
        }
        Ok(Self {
            class_info,
            left_char_model: NgramModel::<String>::deserialize(&mut rdr)?,
            right_char_model: NgramModel::<String>::deserialize(&mut rdr)?,
            self_char_model: NgramModel::<String>::deserialize(&mut rdr)?,
        })
    }
}