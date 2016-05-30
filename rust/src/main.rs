mod rwfile;
mod frand;

use std::env;
use std::io::prelude::*;

use std::thread;
use std::sync::{Arc, Mutex};

extern crate rand;
extern crate num_cpus;
extern crate time;

// Return the position of the minimal value of a vector (the vector must be nonempty)
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

// Compute the overage or lack in each workshop if we put the people into their best wishes
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

    (x, ok) // ok = no lack and no overage
}

fn shuffle(vmin: &Vec<u32>, vmax: &Vec<u32>, mut wishes: Vec<Vec<f64>>, rand: &mut frand::FastRand) -> Vec<usize>
{
    // Modify randomly the actual wishes by adding a value in (-0.5,0.5)
    for i in 0..wishes.len() {
        for j in 0..vmin.len() {
            wishes[i][j] += 0.5 * 2.0 * (rand::random::<f64>() - 0.5);
        }
    }
    // Modify slowly the wishes according to the attractivity of each workshop up to eveybody is in his "first choice" (modified first choice)
    loop {
        let (cnt, ok) = count(&vmin, &vmax, &wishes);
        if ok { break; }

        for i in 0..wishes.len() {
            for j in 0..vmin.len() {
                wishes[i][j] += 3e-4 * rand.get() * (cnt[j]*cnt[j]*cnt[j]) as f64;
            }
        }
    }

    // Extract the results
    let mut results = Vec::with_capacity(vmin.len());

    for i in 0..wishes.len() {
        results.push(min_pos(&wishes[i]));
    }

    results
}

fn action(wishes: &Vec<Vec<u32>>, results: &Vec<usize>) -> i32 {
    let mut score = 0;
    for i in 0..wishes.len() {
        score += (wishes[i][results[i]] * wishes[i][results[i]]) as i32;
    }
    score
}

fn search_solution(vmin: &Vec<u32>, vmax: &Vec<u32>, wishes: &Vec<Vec<u32>>, time: f64) -> Vec<Vec<usize>> {
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
        best_results: Vec<Vec<usize>>,
        iterations:   usize
    };
    let shared = Arc::new(Mutex::new(SharedData {
        timeout:      f64::max(10.0, time),
        best_score:   -1,
        best_results: Vec::new(),
        iterations:   0
    }));

    let mut childs = Vec::new();

    print!("\x1B[31;42m"); // red background green

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
                let score = action(&wishes, &results);

                if rand.get_turns() > 512 {
                    rand.generate();
                }

                let mut shared = shared.lock().unwrap();

                shared.iterations += 1;

                if score < shared.best_score || shared.best_score == -1 {
                    shared.best_score = score;
                    shared.best_results.clear();

                    let now = time::precise_time_s();
                    shared.timeout = now + f64::max(1.5 * (now - t0), time);
                }
                if score == shared.best_score {
                    if !shared.best_results.contains(&results) {
                        shared.best_results.push(results);
                    }
                }
                if id == 0 {
                    print!("\x1B[999D");
                    print!("\x1B[K");
                    print!("Iter {it:>5} ({rate:>4.0}/s). Actual best score : {bs} x {nbs}. {left:>4.1} seconds left ", bs=shared.best_score, nbs=shared.best_results.len(), it=shared.iterations, rate=shared.iterations as f64 / (time::precise_time_s() - t0), left=shared.timeout - time::precise_time_s());
                    std::io::stdout().flush().ok().unwrap();
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

    print!("\x1B[0m"); // reset color
    print!("\x1B[999D");
    print!("\x1B[K");

    shared.best_results.clone()
}

fn main() {
    let help = "arguments: input_file output_file (execution_time) (delimiter)";
    let in_file = env::args().nth(1).expect(help);
    let out_file = match env::args().nth(2) {
        Some(x) => x,
        None => String::new()
    };
    let time = match env::args().nth(3) {
        Some(x) => x.parse::<f64>().unwrap(),
        None => 10.0
    };
    let delimiter : String = match env::args().nth(4) {
        Some(x) => x,
        None => ",".to_string()
    };

    let (vmin, vmax, wishes, ids) = rwfile::read_file(&in_file, &delimiter);

    println!("{} students. {} workshops", wishes.len(), vmin.len());

    let results = search_solution(&vmin, &vmax, &wishes, time);
    assert!(!results.is_empty());
    let score = action(&wishes, &results[0]);

    let mut inc = vec![0; vmin.len()];
    let mut wos = vec![0; vmin.len()];
    for i in 0..wishes.len() {
        inc[wishes[i][results[0][i]] as usize] += 1;
        wos[results[0][i] as usize] += 1;
    }
    println!("{} results with final best score {}", results.len(), score);
    println!("Amount in each choice : {:?}", inc);

    for j in 0..vmin.len() {
        println!("WS{:>2} : {} <= {} <= {}", j+1, vmin[j], wos[j], vmax[j]);
    }

    if out_file.is_empty() {
        for x in &results[0] {
            print!("{}{}", x, delimiter);
        }
        println!("");
    } else {
        rwfile::write_file(&out_file, &results, &wishes, &ids, &delimiter);
        println!("Results written into file {}", out_file);
    }
}
