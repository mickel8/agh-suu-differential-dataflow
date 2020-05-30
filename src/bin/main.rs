use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use differential_dataflow::input::InputSession;
use timely::communication::allocator::generic::Generic;
use timely::worker::Worker;

use agh_suu_differential_dataflow::algorithms::graph::triangles;

fn main() {
    let proc_id = fs::read_to_string("/etc/agh-suu-dd/proc-id.cnf").unwrap().trim_end().to_owned();
    let processes = env::var("AGH_SUU_DD_CLUSTER_SIZE").unwrap();
    println!("Process-ID: {}; Cluster size: {}", proc_id, processes);

    let args = vec!["-w".to_owned(), "1".to_owned(),
                    "-n".to_owned(), processes.to_owned(),
                    "-p".to_owned(), proc_id.to_owned(),
                    "-h".to_owned(), "/etc/agh-suu-dd/hostfile.cnf".to_owned()].into_iter();

    timely::execute_from_args(args, move |worker| {
        let mut input = triangles(worker);
        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
        println!("Server started! Waiting on port 7878");

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Received a request");
            handle_connection(stream, &input, worker)
        }

        println!("End computation");
    }).expect("Computation terminated abnormally");

    println!("Sleep");

    thread::sleep(Duration::from_millis(100000));
}

fn handle_connection(mut stream: TcpStream, input: &InputSession<i32, (i32, i32), isize>, worker: &Worker<Generic>) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        let result = compute(buffer, input, worker);
        if result {
            ("HTTP/1.1 200 OK\r\n\r\n", "Hello")
        } else {
            ("HTTP/1.1 400 BAD REQUEST\r\n\r\n", "Bad request")
        }
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "Not found")
    };

    let response = format!("{}{}", status_line, filename);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn compute(buffer: [u8; 512], input: &InputSession<i32, (i32, i32), isize>, worker: &Worker<Generic>) -> bool {
    // let message = parse(buffer)
    // input.update_at(message.data, message.time, message.diff);
    // let time = message.low_watermark();
    // input.advance_to(time);
    // input.flush();
    // worker.step();
    true
}
