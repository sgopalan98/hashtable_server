use std::{net::TcpStream, io::{BufReader, BufRead, Write}};


pub(crate) fn read_command(stream: &mut TcpStream) -> String{
    let mut input = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut input).unwrap();
    let input: String = input.trim().to_owned();
    return input;
}

pub(crate) fn write_string(stream: &mut TcpStream, output: String) {
    
    stream.write(output.as_bytes()).unwrap();
}