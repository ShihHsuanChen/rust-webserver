use std::io::Cursor;
use std::collections::HashMap;


pub type ContentType = Option<Box<dyn HasContent>>;

#[derive(Clone)]
pub struct FileCursor {
    filename: String,
    cursor: Cursor<Vec<u8>>,
}
impl FileCursor {
    pub fn new(filename: String, bytes: Vec<u8>) -> Self {
        Self {
            filename,
            cursor: Cursor::new(bytes),
        }
    }
    pub fn filename(&self) -> String {
        self.filename.clone()
    }
    pub fn ext(&self) -> String {
        match self.filename.split_once(".") {
            Some((_, ext)) => ext.to_string(),
            None => "".to_string(),
        }
    }
    pub fn cursor(&mut self) -> &Cursor<Vec<u8>> {
        &self.cursor
    }
}


pub enum RawDataType<'a> {
    Text(&'a String),
    Binary(&'a Vec<u8>),
    File(&'a FileCursor),
    Multiple(&'a HashMap<String,Box<dyn HasContent>>),
    None,
}

pub trait HasContent {
    fn content_type(&self) -> &str;
    fn content(&self) -> RawDataType;
}

pub struct FileContent {
    pub filename: String,
    pub filename_encoded: String,
    pub content_type: String,
    pub content: FileCursor,
}
impl HasContent for FileContent {
    fn content_type(&self) -> &str {
        &self.content_type
    }
    fn content(&self) -> RawDataType {
        RawDataType::File(&self.content)
    }
}

pub struct BinaryContent {
    pub content_type: String,
    pub content: Vec<u8>,
}
impl HasContent for BinaryContent {
    fn content_type(&self) -> &str {
        &self.content_type
    }
    fn content(&self) -> RawDataType {
        RawDataType::Binary(&self.content)
    }
}


pub struct TextContent {
    pub content_type: String,
    pub content: String,
}
impl HasContent for TextContent {
    fn content_type(&self) -> &str {
        &self.content_type
    }
    fn content(&self) -> RawDataType {
        RawDataType::Text(&self.content)
    }
}


pub struct FormContent {
    pub content_type: String,
    pub content: HashMap<String,Box<dyn HasContent>>,
}
impl FormContent {
    // pub fn get(&self, key: &str) -> Option<&ContentType> {
    pub fn get(&self, key: &str) -> Option<&Box<dyn HasContent>> {
        self.content.get(key)
    }
}
impl HasContent for FormContent {
    fn content_type(&self) -> &str {
        &self.content_type
    }
    fn content(&self) -> RawDataType {
        RawDataType::Multiple(&self.content)
    }
}


// pub enum ContentType {
//     Text(Text),
//     File(File),
//     Form(Form),
//     Binary(Binary),
//     None,
// }
// impl HasContent for ContentType {
//     fn content_type(&self) -> &str {
//         match self {
//             ContentType::None => "",
//             _ => self.content_type()
//         }
//     }
//     fn content(&self) -> RawDataType {
//         match self {
//             ContentType::None => RawDataType::None,
//             _ => self.content()
//         }
//     }
// }
