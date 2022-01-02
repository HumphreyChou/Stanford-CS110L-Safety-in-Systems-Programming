use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    for _ in 0..input_vec.len() {
        // This is really silly but I could not find a better way
        output_vec.push(Default::default());
    }
    let batch = input_vec.len() / num_threads + 1;
    let (tx, rx) = crossbeam_channel::unbounded();
    let mut threads = Vec::new();
    for _ in 0..num_threads {
        let mut nums = Vec::new();
        for _ in 0..batch {
            match input_vec.pop() {
                Some(num) => nums.push((input_vec.len(), num)),
                None => break
            };
        }
        if nums.len() == 0 {
            break;
        }
        let sender = tx.clone();
        threads.push(thread::spawn(move || {
            loop {
                match nums.pop() {
                    Some((index, num)) => sender
                        .send((index, f(num)))
                        .expect("Sender error in worker threads"),
                    None => break,
                }
            }
            // drop(sender);
        }));
    }
    drop(tx);

    while let Ok((index, res)) = rx.recv() {
        output_vec[index] = res;
    }

    for handle in threads {
        handle.join().expect("Panics when reaping child threads");
    }
    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
