use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use differential_dataflow::input::InputSession;
use spmc;
use spmc::{Receiver, Sender};
use timely::communication::allocator::generic::Generic;
use timely::dataflow::operators::probe::Handle;
use timely::worker::Worker;

use agh_suu_differential_dataflow::algorithms::graph::triangles;
use agh_suu_differential_dataflow::Msg;

fn main() {
    let (tx, rx): (Sender<TcpStream>, Receiver<TcpStream>) = spmc::channel();

    let handle = thread::spawn(move || {
        run_dd_instance(rx);
    });
    run_tcp_server(tx);
    handle.join().unwrap();

    println!("Sleep");
    thread::sleep(Duration::from_millis(100000));
}

fn run_tcp_server(mut sender: Sender<TcpStream>) {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("Server started! Waiting on port 7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Received a new connection");
        sender.send(stream).unwrap();
    }
}

fn run_dd_instance(receiver: Receiver<TcpStream>) {
    let workers = env::var("AGH_SUU_DD_WORKERS_PER_NODE").unwrap();
    let processes = env::var("AGH_SUU_DD_CLUSTER_SIZE").unwrap();
    let proc_id = fs::read_to_string("/etc/agh-suu-dd/proc-id.cnf").unwrap().trim_end().to_owned();
    let hostfile = "/etc/agh-suu-dd/hostfile.cnf";

    println!("Process-ID: {}; Cluster size: {}", proc_id, processes);

    let args = vec!["-w".to_owned(), workers.to_owned(),
                    "-n".to_owned(), processes.to_owned(),
                    "-p".to_owned(), proc_id.to_owned(),
                    "-h".to_owned(), hostfile.to_owned()].into_iter();

    timely::execute_from_args(args, move |worker| {
        println!("Worker {} of process {} started", worker.index(), proc_id);

        let (mut input, probe) = triangles(worker);
        loop {
            let stream = receiver.recv().unwrap();
            handle_connection(stream, &mut input, &probe, worker)
        }
    }).expect("Computation terminated abnormally");
}

fn handle_connection(
    mut stream: TcpStream,
    input: &mut InputSession<i32, (i32, i32), isize>,
    probe: &Handle<i32>,
    worker: &mut Worker<Generic>,
) {
    println!("Worker {}: Waiting for msg", worker.index());
    loop {
        let mut buffer = [0; 512];
        let bytes = stream.read(&mut buffer).unwrap();
        if bytes == 0 {
            input.advance_to(input.time() + 1);
            input.flush();
            while probe.less_than(&input.time()) {
                worker.step();
            }
            println!("Worker {}: Connection closed", worker.index());
            stream.shutdown(Shutdown::Both).unwrap();
            break;
        }
        let mut index= 1;
        while index < bytes {
            let size: u8 = bincode::deserialize(&buffer[index - 1..index]).unwrap();
            let msg: Msg = bincode::deserialize(&buffer[index..(index + size as usize)]).unwrap();
            println!("Worker {}: Got msg: {}", worker.index(), msg);
            compute(msg, input);
            index += size as usize + 1;
        }
        stream.flush().unwrap();
    }
}

fn compute(
    msg: Msg,
    input: &mut InputSession<i32, (i32, i32), isize>,
) {
    match msg {
        Msg::Add(edge) => {
            input.insert(edge);
            input.flush();
        },
        Msg::Remove(edge) => {
            input.remove(edge);
            input.flush();
        }
    }
}
