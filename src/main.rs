extern crate xml;
extern crate regex;
extern crate encoding;

pub mod util;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::fs;
use std::io::{Read, Write};
use std::env;

use xml::reader::EventReader;
use xml::writer::EmitterConfig;
use xml::reader::XmlEvent;
use xml::writer::events::XmlEvent as WriterXmlEvent;
use encoding::types::EncodingRef;
use encoding::all::UTF_8;

use util::{SafeWrite, new_line_count};


fn main() {
	let args: Vec<_> = env::args().collect();

    if args.len() <= 2 {
        println!("USAGE: xmlint file.xml file_result.xml");
        return;
    }

	let file_name = &args[1];
    let file_name_target = &args[2];
    let mut file_name_result = file_name_target.to_string();
    let encoding = detect_encoding(&file_name).expect("cannot detect encoding");
    let mut convert = false;
    let str: String; 

    println!("Detected encoding: {}", encoding.name());
   
    let read : Box<Read> = match encoding.name() {
    	"utf-8" => {	
    		Box::new(BufReader::new(File::open(file_name).unwrap()))
    	}
    	_ => {
    		convert = true;
    		file_name_result.push_str(".tmp");
		    str = decode(file_name, &encoding);
		    Box::new(str.as_bytes())
    	}
    };

    let reader = EventReader::new(read);
    let mut file_result = File::create(&file_name_result).unwrap();
    let mut writer = EmitterConfig::default().perform_indent(true).create_writer(&mut file_result);

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

    if convert {
    	encode(&file_name_result, &file_name_target, &encoding);
    	fs::remove_file(file_name_result).expect("Deleting file failed");
    }

    println!("File converted to {}", file_name_target);
}

fn detect_encoding(file_name: &str) -> Option<EncodingRef> {
    use regex::Regex;
    use encoding::label::encoding_from_whatwg_label;

    let re = Regex::new("encoding\\s*=\\s*['\"]([^'\"]+)['\"]").unwrap();
    let file = File::open(file_name).unwrap();    
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

/// decode file into utf8 file
fn decode(file_name: &str, inenc: &EncodingRef) -> String {
	use encoding::{DecoderTrap};

	println!("Decoding({}) {}", inenc.name(), file_name);
	let mut file = File::open(file_name).unwrap();
	let mut ret = Vec::new();
	let ll = file.read_to_end(&mut ret);

	println!("File size {}", ll.unwrap());

	let decoded = match inenc.decode(&ret, DecoderTrap::Strict) {
        Ok(s) => s,
        Err(e) => panic!("decoder error: {}", e),
    };

    decoded
}

// encode file into specified encoding
fn encode(file_name: &str, file_name_result: &str, outenc: &EncodingRef) {
	use encoding::{EncoderTrap};

	println!("Encoding ({}) {}", outenc.name(), file_name_result);

	let mut file_in = File::open(file_name).unwrap();
	let mut content = String::new();
	file_in.read_to_string(&mut content).unwrap();
	let mut file_out = File::create(file_name_result).unwrap();

	let encoded = match outenc.encode(&content, EncoderTrap::Strict) {
        Ok(s) => s,
        Err(e) => panic!("encoder error: {}", e),
    };

    file_out.write_all(&encoded).expect(&format!("File write failed: {}", file_name_result));
}
