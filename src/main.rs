use rand::prelude::*;
// PIDs 0 and 1 can't be reassigned.
const PID_SPACE: usize = (1 << 24) - 2;
const SIMULATIONS: usize = 1000;

fn main() {
    let mut tries_existing_process = vec![0.0; PID_SPACE];
    eprint!("Simulation ");
    for s in 0..SIMULATIONS {
        eprint!("{s} ");
        let mut pids = vec![false; PID_SPACE];
        for i in 0..PID_SPACE {
            let (_, tries) = tryassign(&mut pids);
            tries_existing_process[i] += tries as f64;
        }
    }
    let tries_existing_process_avg: Vec<f64> = tries_existing_process.into_iter().map(|n| n / SIMULATIONS as f64).collect();
    eprintln!("");
    println!("{tries_existing_process_avg:?}");
}

fn tryassign(pids: &mut [bool]) -> (usize, usize) {
    let mut rng = thread_rng();
    let mut success = false;
    let mut tries = 0;
    let mut candidate = 0;
    while !success {
        tries += 1;
        candidate = rng.gen_range(0..PID_SPACE);
        if !pids[candidate] {
            pids[candidate] = true;
            success = true;
        }
    }
    (candidate, tries)
}
