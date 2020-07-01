use std::io;
use std::io::{Write, BufReader, BufRead};
use std::net::{TcpStream, Shutdown};
use agh_suu_differential_dataflow::{Msg, Edge, Time};
use std::fs::File;

fn main() {
    let server_addr = std::env::args().nth(1).unwrap();
    println!("Server address {}", server_addr);
    println!("Type msg (op [v1 v2] time; e.g. + 1 2 0 or = 1): ");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.split_whitespace().collect();
        if *input.get(0).unwrap() == "f" {
            let path = input.get(1).unwrap();
            let batch: i32 = input.get(2).unwrap().parse().unwrap();
            let mut buf_reader = BufReader::new(File::open(path).unwrap());
            let mut line = String::new();
            let mut msgs = vec![];
            while buf_reader.read_line(&mut line).unwrap() != 0 {
                print!("line: {}", line);
                msgs.push(parse(line.split_whitespace().collect()));
                line = String::new();
                if msgs.len() as i32 == batch {
                    send(&server_addr, msgs);
                    msgs = vec![];
                }
            }

            // for remaining messages
            if msgs.len() != 0 {
                send(&server_addr, msgs);
            }
        } else {
            send(&server_addr, vec![parse(input)]);
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

fn send(server_addr: &String, msgs: Vec<Msg>) {
    let mut stream = TcpStream::connect(server_addr).unwrap();
    for msg in msgs {
        let msg_s = serialize(msg);
        stream.write(&msg_s).unwrap();
    }
    stream.flush().unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}

fn serialize(msg: Msg) -> Vec<u8> {
    let mut buf = &mut bincode::serialize(&msg).unwrap();
    let msg_s = &mut bincode::serialize(&(buf.len() as u8)).unwrap();
    msg_s.append(&mut buf);
    msg_s.to_vec()
}
