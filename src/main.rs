extern crate differential_dataflow;
extern crate timely;

pub use crate::algorithms::graph::triangles;

mod algorithms;

fn main() {
    timely::execute_from_args(std::env::args(), move |worker| {
        let mut input = triangles(worker);

        input.advance_to(0);
        let edges = vec![(1, 2), (2, 3), (1, 3), (2, 1), (3, 2), (3, 1), (1, 4),
                         (4, 1), (2, 5), (5, 2), (7, 8), (8, 7), (7, 9), (9, 7),
                         (8, 9), (8, 9), (2, 8), (8, 2)];
        for (u, v) in edges {
            input.insert((u, v));
        }
    }).expect("Computation terminated abnormally");
}