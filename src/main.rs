extern crate xml;

pub mod util;

use std::fs::File;
use std::io::BufReader;

use xml::reader::EventReader;
use xml::writer::EmitterConfig;
use xml::reader::XmlEvent;
use xml::writer::events::XmlEvent as WriterXmlEvent;

use util::{SafeWrite, new_line_count};

fn main() {
    let file_in = File::open("..\\docs\\file.xml").unwrap();    
    let mut file_out = File::create("..\\file_result.xml").unwrap();

    let reader = EventReader::new(BufReader::new(&file_in));
    let mut writer = EmitterConfig::default().perform_indent(true).create_writer(&mut file_out);

    for ev in reader {
        match ev {
            Ok(XmlEvent::Whitespace(ref data)) => {
                // a little bit hacky)                
                if new_line_count(data) >= 2 {
                    writer.write_safe(WriterXmlEvent::Characters("\r\n\r\n\t"));
                }
            }
            Ok(ev) => {
                if let Some(ev) = ev.as_writer_event() {
                    writer.write_safe(ev);
                }
            }
            Err(ev) => panic!("Error: {:?}", ev),
        }
    }
}
