use xml::writer::events::XmlEvent as WriterXmlEvent;

use std::io::Write;
use std::str;

use xml::writer::EventWriter;

pub fn new_line_count(st: &str) -> i32 {
    st.chars().filter(|&x| x == '\n').count() as i32
}

pub trait SafeWrite {
    fn write_safe(&mut self, event: WriterXmlEvent);
}


impl<W: Write> SafeWrite for EventWriter<W> {
    fn write_safe(&mut self, event: WriterXmlEvent) {
        match self.write(event) {
            Ok(_) => {}

            Err(ev) => panic!("Writer error: {:?}", ev),
        }
    }
}