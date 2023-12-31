use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;
use std::{fs::File, io::Write};

fn main() {
    // Write
    println!("\nWrite");
    let data = b"ABCDE";
    let mut buffer = File::create("hoge.txt").unwrap();

    let mut pos = 0;
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..]).unwrap();
        pos += bytes_written;
        println!("pos: {} bytes_written: {}", pos, bytes_written);
    }

    // Read
    println!("\nRead");
    let mut file = File::open("./src/write_read_cursor/fuga.txt").unwrap();
    let mut buffer = [0; 10];

    file.read_exact(&mut buffer).unwrap();
    println!("file: {:?}, buffer: {:?}", file, buffer);

    // Cursor
    println!("\nCursor");
    let mut file = File::open("./src/write_read_cursor/fuga.txt").unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let mut cursor = Cursor::new(data);
    let mut buffer = [0; 10];

    println!("pos: {:?}, buffer: {:?}", cursor.position(), buffer);
    cursor.read_exact(&mut buffer).unwrap();
    println!("pos: {:?}, buffer: {:?}", cursor.position(), buffer);
    cursor.seek(SeekFrom::Start(2)).unwrap();
    println!("pos: {:?}, buffer: {:?}", cursor.position(), buffer);

    cursor.set_position(0);
    cursor.write_all(b"ZZZZZZ").unwrap();
    println!(
        "cursor: {:?}",
        String::from_utf8_lossy(&cursor.into_inner())
    );
}
