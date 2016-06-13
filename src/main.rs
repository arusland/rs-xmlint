extern crate xml;
extern crate regex;
extern crate encoding;

pub mod util;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::*;

use xml::reader::EventReader;
use xml::writer::EmitterConfig;
use xml::reader::XmlEvent;
use xml::writer::events::XmlEvent as WriterXmlEvent;
use encoding::types::EncodingRef;
use encoding::all::UTF_8;

use util::{SafeWrite, new_line_count};


fn main() {
    let file_in = File::open("d:\\WORK\\MyProjects\\rust\\rs-xmlint\\docs\\file_1251.xml").unwrap();    
    let mut file_out = File::create("..\\file_result.xml").unwrap();
    let encoding = detect_encoding(&file_in);

    println!("encoding: {}", encoding.expect("cannot detect encoding").name());
    // TODO: detect source encoding
    // convert to utf8
    // format xml
    // convert to source encoding
    // save result xml

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

fn detect_encoding(file: &File) -> Option<EncodingRef> {
    use regex::Regex;
    use encoding::label::encoding_from_whatwg_label;

    let re = Regex::new("encoding\\s*=\\s*['\"]([^'\"]+)['\"]").unwrap();
    let file = BufReader::new(file);

    // check only first line
    if let Some(line) = file.lines().next() {
        if let Some(caps) = re.captures(&line.unwrap()) {
            let encoding = caps.at(1).unwrap();

            return encoding_from_whatwg_label(encoding);
        }
    }

    Some(UTF_8 as EncodingRef)
}
