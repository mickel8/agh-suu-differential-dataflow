use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::time::Duration;

use differential_dataflow::input::InputSession;
use timely::communication::allocator::generic::Generic;
use timely::worker::Worker;

use agh_suu_differential_dataflow::algorithms::graph::triangles;
use timely::dataflow::operators::probe::Handle;
use agh_suu_differential_dataflow::Msg;

fn main() {
    let proc_id = fs::read_to_string("/etc/agh-suu-dd/proc-id.cnf").unwrap().trim_end().to_owned();
    let processes = env::var("AGH_SUU_DD_CLUSTER_SIZE").unwrap();
    println!("Process-ID: {}; Cluster size: {}", proc_id, processes);

    let args = vec!["-w".to_owned(), "1".to_owned(),
                    "-n".to_owned(), processes.to_owned(),
                    "-p".to_owned(), proc_id.to_owned(),
                    "-h".to_owned(), "/etc/agh-suu-dd/hostfile.cnf".to_owned()].into_iter();

    timely::execute_from_args(args, move |worker| {
        let (mut input, probe) = triangles(worker);
        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
        println!("Server started on worker {}! Waiting on port 7878", worker.index());

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Received a new connection");
            handle_connection(stream, &mut input, &probe, worker)
        }

        println!("End computation");
    }).expect("Computation terminated abnormally");

    println!("Sleep");

    thread::sleep(Duration::from_millis(100000));
}

fn handle_connection(
    mut stream: TcpStream,
    input: &mut InputSession<i32, (i32, i32), isize>,
    probe: &Handle<i32>,
    worker: &mut Worker<Generic>,
) {
    println!("Waiting for msg");
    loop {
        let mut buffer = [0; 512];
        let bytes = stream.read(&mut buffer).unwrap();
        if bytes == 0 {
            println!("Connection closed");
            stream.shutdown(Shutdown::Both).unwrap();
            break;
        }
        let mut index= 1;
        while index < bytes {
            let size: u8 = bincode::deserialize(&buffer[index - 1..index]).unwrap();
            let msg: Msg = bincode::deserialize(&buffer[index..(index + size as usize)]).unwrap();
            println!("Got msg: {}", msg);
            compute(msg, input, probe, worker);
            index += size as usize + 1;
        }
        stream.flush().unwrap();
    }
}

fn compute(
    msg: Msg,
    input: &mut InputSession<i32, (i32, i32), isize>,
    probe: &Handle<i32>,
    worker: &mut Worker<Generic>,
) {
    match msg {
        Msg::Add(edge, time) => {
            input.advance_to(time);
            input.insert(edge);
            input.flush();
        },
        Msg::Remove(edge, time) => {
            input.advance_to(time);
            input.remove(edge);
            input.flush();
        },
        Msg::Result(time) => {
            input.advance_to(time);
            input.flush();
            while probe.less_than(&input.time()) {
                worker.step();
            }
        },
    }
}
