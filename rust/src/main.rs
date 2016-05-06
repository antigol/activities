mod rwfile;
mod frand;

use std::env;
use std::io::prelude::*;
use std::io;

extern crate time;

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

fn is_null(xs: &Vec<i32>) -> bool {
    for i in 0..xs.len() {
        if xs[i] != 0 {
            return false;
        }
    }
    true
}

fn count(vmin: &Vec<u32>, vmax: &Vec<u32>, wishes: &Vec<Vec<f64>>) -> Vec<i32> {
    let mut x: Vec<i32> = vec![0; vmin.len()];

    for i in 0..wishes.len() {
        x[min_pos(&wishes[i])] += 1;
    }

    for i in 0..vmin.len() {
        if x[i] < vmin[i] as i32 {
            x[i] -= vmin[i] as i32; // negative value for a lack
        } else if x[i] > vmax[i] as i32 {
            x[i] -= vmax[i] as i32; // positive value for an overage
        } else {
            x[i] = 0; // null value if in range
        }
    }

    x
}

fn shuffle(vmin: &Vec<u32>, vmax: &Vec<u32>, mut wishes: Vec<Vec<f64>>, rand: &mut frand::FastRand) -> Vec<usize>
{
    for i in 0..wishes.len() {
        for j in 0..wishes[i].len() {
            wishes[i][j] += 2.0 * 0.1 * (rand.get() - 0.5);
        }
    }
    let mut cnt = count(&vmin, &vmax, &wishes);

    while !is_null(&cnt) {
        for i in 0..wishes.len() {
            for j in 0..vmin.len() {
                wishes[i][j] += 2e-4 * (cnt[j] as f64) * rand.get();
            }
        }
        cnt = count(&vmin, &vmax, &wishes);
    }

    let mut results = vec![0; wishes.len()];

    for i in 0..wishes.len() {
        results[i] = min_pos(&wishes[i]);
    }

    results
}

fn search_solution(vmin: &Vec<u32>, vmax: &Vec<u32>, wishes: &Vec<Vec<u32>>) -> Vec<usize> {
    let mut rand = frand::FastRand::new();
    rand.initialize();

    let mut best_score: i32 = -1;
    let mut best_results = Vec::new();

    let mut wishesf = vec![vec![0.0; vmin.len()]; wishes.len()];
    for i in 0..wishes.len() {
        for j in 0..wishes[i].len() {
            wishesf[i][j] = wishes[i][j] as f64;
        }
    }

    let mut timeout = 10.0;
    let t0 = time::precise_time_s();
    let mut t1 = t0;

    while time::precise_time_s() - t1 < timeout {
        print!("{:.1} seconds left      \r", t1 + timeout - time::precise_time_s());
        io::stdout().flush().ok().expect("Could not flush stdout");
        let results = shuffle(&vmin, &vmax, wishesf.clone(), &mut rand);

        let mut score: i32 = 0;
        for i in 0..wishes.len() {
            score += (wishes[i][results[i]] * wishes[i][results[i]]) as i32;
        }

        if score < best_score || best_score == -1 {
            best_score = score;
            best_results = results;
            println!("best score : {}      ", score);
            timeout = f64::max(1.5 * (time::precise_time_s() - t0), 30.0);
            t1 = time::precise_time_s();
        }
    }
    print!("                        \r");

    best_results
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
}
