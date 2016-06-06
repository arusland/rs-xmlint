extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::str;

use xml::reader::EventReader;
use xml::writer::EmitterConfig;
use xml::reader::XmlEvent;


fn main() {
    let file_in = File::open("..\\docs\\file.xml").unwrap();
    let mut file_out = File::create("..\\file_result.xml").unwrap();

    let reader = EventReader::new(BufReader::new(&file_in));
    let mut writer = EmitterConfig::default().perform_indent(true).create_writer(&mut file_out);

    for ev in reader {
        match ev {
            Ok(XmlEvent::Whitespace(ref data)) => {
                /*println!("Whitespace: {:?}", data);

                if "\r\n\r\n" == data {
                    match writer.write(xml::writer::events::XmlEvent::Characters(data)) {
                        Ok(_) => {}

                        Err(ev) => panic!("Writer error: {:?}", ev),
                    }
                }*/
            }
            Ok(ev) => {
                if let Some(ev) = ev.as_writer_event() {
                    match writer.write(ev) {
                        Ok(_) => {}

                        Err(ev) => panic!("Writer error: {:?}", ev),
                    }
                }
            }
            Err(ev) => panic!("Error: {}", ev),
        }
    }
}