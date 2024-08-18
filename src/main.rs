use rand::prelude::*;
use bitvec::prelude::*;
use bitvec::bitvec;
use clap::Parser;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    pid_space: usize,

    #[arg(short, long)]
    simulations: usize,

    #[arg(short, long)]
    quiet: bool,

    #[arg(short, long)]
    no_output: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut tries_existing_process = vec![0.0; cli.pid_space];
    if !cli.quiet {
        eprint!("Simulation ");
    }
    for s in 0..cli.simulations {
        if !cli.quiet {
            eprint!("{s} ");
        }
        let mut pids = bitvec![0; cli.pid_space];
        for i in 0..cli.pid_space {
            let (_, tries) = tryassign(&mut pids, cli.pid_space);
            tries_existing_process[i] += tries as f64;
        }
    }
    let tries_existing_process_avg: Vec<f64> = tries_existing_process.into_iter().map(|n| n / cli.simulations as f64).collect();
    eprintln!("");
    if !cli.no_output {
        println!("{tries_existing_process_avg:?}");
    }
}

fn tryassign(pids: &mut BitVec, pid_space: usize) -> (usize, usize) {
    let mut rng = thread_rng();
    let mut success = false;
    let mut tries = 0;
    let mut candidate = 0;
    while !success {
        tries += 1;
        candidate = rng.gen_range(0..pid_space);
        if !pids[candidate] {
            pids.set(candidate, true);
            success = true;
        }
    }
    (candidate, tries)
}
