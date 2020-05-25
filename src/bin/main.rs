use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use agh_suu_differential_dataflow::algorithms::graph::triangles;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        compute(); // TODO: pass data from request and return results to response
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn compute() {
    timely::execute_from_args(std::env::args(), move |worker| {
        let mut input = triangles(worker);

        /*
        3--2----8--9
        | / \   | /
        |/   5  |/
        1--4    7
        */
        input.advance_to(0);
        let edges = vec![(1, 2), (2, 3), (1, 3), (2, 1), (3, 2), (3, 1), (1, 4),
                         (4, 1), (2, 5), (5, 2), (7, 8), (8, 7), (7, 9), (9, 7),
                         (8, 9), (8, 9), (2, 8), (8, 2)];
        for (u, v) in edges {
            input.insert((u, v));
        }

        input.advance_to(1);
        /*
        3  2----8--9
        | /|\   | /
        |/ | 5  |/
        1--4    7
        */
        input.insert((2, 4));
        input.insert((4, 2));
        input.remove((2, 3));
        input.remove((3, 2));
    }).expect("Computation terminated abnormally");
}
