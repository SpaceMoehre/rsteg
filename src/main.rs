use std::fmt;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::str::Bytes;

#[derive(Debug)]
struct Chunk{
    length: u32,
    ctype: Vec<u8>,
    data: Vec<u8>,
    crc: u32
}

// impl std::fmt::Display for Chunk {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
//         return write!(f, "Chunk: length: {}, ctype: {}, data: {}... , crc: {}", self.length, self.ctype, self.data.try_into().unwrap(), self.crc);
//     }
// }

fn main() {
    let mut raw_png_data = fs::read("indeep.png").unwrap();
    let mut chunks: Vec<Chunk> = Vec::new();
    let pngheader = raw_png_data[..8].to_vec();
    assert_eq!(pngheader, vec![137, 80, 78, 71, 13, 10, 26, 10]); //correct png header
    raw_png_data = raw_png_data[8..].to_vec();
    while raw_png_data.clone().into_iter().len() > 0{
        let png_data = raw_png_data.clone();
        let length = u32::from_be_bytes(png_data[..4].try_into().expect("could not parse remaining raw_png_data"));
        let ctype = png_data[4..8].to_vec();
        let data = png_data[8..8 + length as usize].to_vec();
        let crc = u32::from_be_bytes(png_data[8 + length as usize..12 + length as usize].try_into().expect("could not parse remaining raw_png_data"));
        let chunk = Chunk{length,ctype,data,crc};
        //println!("{chunk:?}");
        chunks.push(chunk);
        raw_png_data = raw_png_data[12 + length as usize..].to_vec();
    }

    
}

