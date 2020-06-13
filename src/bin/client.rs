use std::io;
use std::io::{Write, BufReader, BufRead};
use std::net::TcpStream;
use agh_suu_differential_dataflow::{Msg, Edge, Time};
use std::fs::File;

fn main() {
    let server_addr = std::env::args().nth(1).unwrap();
    println!("Connecting to {}", server_addr);
    let mut stream = TcpStream::connect(server_addr).unwrap();
    println!("Type msg (op [v1 v2] time; e.g. + 1 2 0 or = 1): ");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.split_whitespace().collect();
        if *input.get(0).unwrap() == "f" {
            let path = input.get(1).unwrap();
            let file = File::open(path).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut line = String::new();
            while buf_reader.read_line(&mut line).unwrap() != 0 {
                print!("line: {}", line);
                let msg = parse(line.split_whitespace().collect());
                send(&mut stream, msg);
                line = String::new();
            }
        } else {
            let msg = parse(input);
            send(&mut stream, msg);
        }
    }
}

fn parse(msg: Vec<&str>) -> Msg {
    let op = msg.get(0).unwrap();
    if *op == "=" {
        Msg::Result(msg.get(1).unwrap().parse().unwrap())
    } else if *op =="+" {
        let (edge, time) = parse_op_args(msg);
        Msg::Add(edge, time)
    } else if *op == "-" {
        let (edge, time) = parse_op_args(msg);
        Msg::Remove(edge, time)
    } else {
        panic!("Unknown operation. Allowed: +, -, =, f");
    }
 }

fn parse_op_args(msg: Vec<&str>) -> (Edge, Time){
    let v1 = msg.get(1).unwrap().parse().unwrap();
    let v2 = msg.get(2).unwrap().parse().unwrap();
    let time = msg.get(3).unwrap().parse().unwrap();
    ((v1, v2), time)
}

fn send(stream: &mut TcpStream, msg: Msg) {
    let mut buf = &mut bincode::serialize(&msg).unwrap();
    let msg = &mut bincode::serialize(&(buf.len() as u8)).unwrap();
    msg.append(&mut buf);
    stream.write(msg).unwrap();
    stream.flush().unwrap();
}
