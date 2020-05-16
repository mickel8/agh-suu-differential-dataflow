extern crate differential_dataflow;
extern crate timely;

pub use crate::algorithms::graph::triangles;

mod algorithms;

fn main() {
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