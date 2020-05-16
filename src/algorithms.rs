extern crate differential_dataflow;
extern crate timely;


pub mod graph {
    use differential_dataflow::input::InputSession;
    use differential_dataflow::operators::{Join, Threshold};
    use timely::communication::allocator::generic::Generic;
    use timely::worker::Worker;

    fn sort_tuple(tuple: (i32, i32, i32)) -> (i32, i32, i32) {
        let mut as_vec: Vec<i32> = vec!(tuple.0, tuple.1, tuple.2);
        as_vec.sort();
        (as_vec[0], as_vec[1], as_vec[2])
    }

    pub fn triangles(worker: &mut Worker<Generic>) -> InputSession<i32, (i32, i32), isize> {
        let mut input: InputSession<i32, (i32, i32), isize> = InputSession::new();

        worker.dataflow(|scope| {
            let edges = input.to_collection(scope);

            edges
                .map(|(x, y)| (y, x))
                .join(&edges)
                .map(|(y, xz)| (xz, y))
                .semijoin(&edges)
                .map(|((x, z), y)| sort_tuple((x, y, z)))
                .distinct()
                .inspect(|x| println!("{:?}", x));
        });
        input
    }
}