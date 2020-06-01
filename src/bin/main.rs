use agh_suu_differential_dataflow::algorithms::graph::triangles;
use rand::prelude::*;

fn main() {
    let num = 100_000;
    let edges = rand_edges(num);
    timely::execute_from_args(std::env::args(), move |worker| {
        let (mut input, probe) = triangles(worker);
        input.advance_to(0);
        let mut i = worker.index() as i32;
        while i < num / 2 {
            input.insert(edges[i as usize]);
            input.advance_to(i);
            i += worker.peers() as i32;
        }
        input.advance_to(i);
        input.flush();
        while probe.less_than(&input.time()) { worker.step(); }
        println!("{:?}\t Step 1 (added {} edges)", worker.timer().elapsed(), num / 2 / worker.peers() as i32);

        while i < num {
            input.insert(edges[i as usize]);
            input.advance_to(i);
            i += worker.peers() as i32;
        }
        input.flush();
        while probe.less_than(&input.time()) { worker.step(); }
        println!("{:?}\t Step 2 (added {} edges)", worker.timer().elapsed(), num / 2 / worker.peers() as i32);
    }).expect("Computation terminated abnormally");

    fn rand_edges(num: i32) -> Vec<(i32, i32)> {
        let mut rng = rand::thread_rng();
        let mut rand_edges = vec![];
        let rnd_bound = (num as f32).sqrt().floor() as i32;
        for _ in 0..num {
            let v1 = rng.gen_range(0, rnd_bound);
            let v2 = rng.gen_range(0, rnd_bound);
            rand_edges.push((v1, v2));
        }
        rand_edges
    }
}
