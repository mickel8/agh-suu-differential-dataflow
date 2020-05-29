use agh_suu_differential_dataflow::algorithms::graph::triangles;
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;

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

        let id = worker.index() as i32;

        input.advance_to(0);
        input.insert((id, id+1));
        input.insert((id+1, id));
        input.insert((id, id+2));
        input.insert((id+2, id));

        input.flush();
        while probe.less_than(&input.time()) { worker.step(); }

        input.advance_to(1);

        if id > 1
        {
            input.remove((id, id-2));
            input.remove((id-2, id));
        }
        input.flush();

        while probe.less_than(&input.time()) { worker.step(); }
        input.advance_to(2);
        input.flush();
        while probe.less_than(&input.time()) { worker.step(); }

        println!("End computation");

    }).expect("Computation terminated abnormally");

    println!("Sleep");

    thread::sleep(Duration::from_millis(100000));
}
