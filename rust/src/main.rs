mod rwfile;
mod frand;

use std::env;
use std::io::prelude::*;
use std::io;

use std::thread;
use std::sync::{Arc, Mutex};

extern crate num_cpus;
extern crate time;

fn clearline() {
    print!("\x1B[999D\x1B[K");
}

fn min_pos<T: PartialOrd + Copy>(xs: &Vec<T>) -> usize {
    let mut k = 0;
    let mut min = xs[0];
    for i in 1..xs.len() {
        if xs[i] < min {
            min = xs[i];
            k = i;
        }
    }
    k
}

fn count(vmin: &Vec<u32>, vmax: &Vec<u32>, wishes: &Vec<Vec<f64>>) -> (Vec<i32>, bool) {
    let mut x = vec![0; vmin.len()];

    for i in 0..wishes.len() {
        x[min_pos(&wishes[i])] += 1;
    }

    let mut ok = true;
    for i in 0..vmin.len() {
        if x[i] < vmin[i] as i32 {
            x[i] -= vmin[i] as i32; // negative value for a lack
            ok = false;
        } else if x[i] > vmax[i] as i32 {
            x[i] -= vmax[i] as i32; // positive value for an overage
            ok = false;
        } else {
            x[i] = 0; // null value if in range
        }
    }

    (x, ok)
}

fn shuffle(vmin: &Vec<u32>, vmax: &Vec<u32>, mut wishes: Vec<Vec<f64>>, rand: &mut frand::FastRand) -> Vec<usize>
{
    for i in 0..wishes.len() {
        for j in 0..vmin.len() {
            wishes[i][j] += 0.1 * 2.0 * (rand.get() - 0.5);
        }
    }
    loop {
        let (cnt, ok) = count(&vmin, &vmax, &wishes);
        if ok { break; }

        for i in 0..wishes.len() {
            for j in 0..vmin.len() {
                wishes[i][j] += 3e-4 * rand.get() * (cnt[j]*cnt[j]*cnt[j]) as f64;
            }
        }
    }

    let mut results = Vec::with_capacity(vmin.len());

    for i in 0..wishes.len() {
        results.push(min_pos(&wishes[i]));
    }

    results
}

fn search_solution(vmin: &Vec<u32>, vmax: &Vec<u32>, wishes: &Vec<Vec<u32>>) -> Vec<usize> {
    let mut wishesf = vec![vec![0.0; vmin.len()]; wishes.len()];
    for i in 0..wishes.len() {
        for j in 0..vmin.len() {
            wishesf[i][j] = wishes[i][j] as f64;
        }
    }

    let t0 = time::precise_time_s();

    struct SharedData {
        timeout:      f64,
        best_score:   i32,
        best_results: Vec<usize>,
        iterations:   usize
    };
    let shared = Arc::new(Mutex::new(SharedData {
        timeout:      10.0,
        best_score:   -1,
        best_results: Vec::new(),
        iterations:   0
    }));

    let mut childs = Vec::new();

    for id in 0..num_cpus::get() {
        let shared = shared.clone();
        let vmin = vmin.clone();
        let vmax = vmax.clone();
        let wishes = wishes.clone();
        let wishesf = wishesf.clone();


        childs.push(thread::spawn(move || {
            let mut rand = frand::FastRand::new();

            loop {
                let results = shuffle(&vmin, &vmax, wishesf.clone(), &mut rand); // all the load is here
                let mut score: i32 = 0;
                for i in 0..wishes.len() {
                    score += (wishes[i][results[i]] * wishes[i][results[i]]) as i32;
                }
                if rand.get_turns() > 512 {
                    rand.initialize();
                }

                let mut shared = shared.lock().unwrap();

                shared.iterations += 1;

                if score < shared.best_score || shared.best_score == -1 {
                    clearline();
                    println!("{}# best score : {}", id, score);
                    shared.best_score = score;
                    shared.best_results = results;

                    let now = time::precise_time_s();
                    shared.timeout = now + f64::max(1.5 * (now - t0), 10.0);
                }
                if id == 0 {
                    clearline();
                    print!("{:>5} ({:.0}/s) {:.1} left", shared.iterations, shared.iterations as f64 / (time::precise_time_s() - t0), shared.timeout - time::precise_time_s());
                    io::stdout().flush().ok().unwrap();
                }
                if time::precise_time_s() > shared.timeout {
                    break;
                }
                /*if shared.iterations > 200 {
                    break;
                }*/
            }
        }));
    }
    for child in childs {
        child.join().unwrap();
    }

    let shared = shared.clone();
    let shared = shared.lock().unwrap();

    clearline();

    shared.best_results.clone()
}

fn main() {
    let help = "arguments: input_file output_file (delimiter)";
    let in_file = env::args().nth(1).expect(help);
    let out_file = env::args().nth(2).expect(help);
    let delimiter : String = match env::args().nth(3) {
        Some(x) => x,
        None => ",".to_string()
    };

    let (vmin, vmax, wishes) = rwfile::read_file(&in_file, &delimiter);

    let results = search_solution(&vmin, &vmax, &wishes);

    rwfile::write_file(&out_file, &delimiter, &vmin, &vmax, &wishes, &results);

    let mut inc = vec![0; vmin.len()];
    let mut wos = vec![0; vmin.len()];
    for i in 0..wishes.len() {
        inc[wishes[i][results[i]] as usize] += 1;
        wos[results[i] as usize] += 1;
    }
    println!("amount in each choice : {:?} over {}", inc, wishes.len());

    for j in 0..vmin.len() {
        println!("WS{:>2} : {} <= {} <= {}", j+1, vmin[j], wos[j], vmax[j]);
    }
}
