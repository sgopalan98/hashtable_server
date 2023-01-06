use std::{net::TcpStream, io::{BufReader, BufRead, Write}};


pub fn read_command(stream: &mut TcpStream, reader: &mut BufReader<TcpStream>) -> String{
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let input: String = input.trim().to_owned();
    return input;
}

pub fn write_string(stream: &mut TcpStream, output: String) {
    stream.write(output.as_bytes()).unwrap();
}