use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

pub fn write_file(out_file: &String, results: &Vec<Vec<usize>>, wishes: &Vec<Vec<u32>>, ids: &Vec<String>, delimiter: &String) {
    let mut buffer = File::create(out_file).expect("error when open output file");

    for i in 0..wishes.len() {
        write!(buffer, "{}", ids[i]).unwrap();
        for k in 0..results.len() {
            let r = results[k][i];
            write!(buffer, "{d}{r}{d}{s}", d=delimiter, r=r, s=wishes[i][r]).unwrap();
        }
        write!(buffer, "\n").unwrap();
    }
}

pub fn read_file(in_file: &String, delimiter: &String) -> (Vec<u32>, Vec<u32>, Vec<Vec<u32>>, Vec<String>) {
    let mut line_no = 0;

    let f = File::open(in_file).expect("error when open file");
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    line_no += 1;
    reader.read_line(&mut buffer).expect("The file is too short");
    let vmin : Vec<u32> = buffer.split(delimiter)
    .map(|x| x.trim())
    .filter(|x| !x.is_empty())
    .map(|x| x.trim().parse::<u32>().expect("error while reading VMIN"))
    .collect();

    let n = vmin.len();

    buffer.clear();
    line_no += 1;
    reader.read_line(&mut buffer).expect("The file is too short");
    let vmax : Vec<u32> = buffer.split(delimiter)
    .map(|x| x.trim())
    .filter(|x| !x.is_empty())
    .map(|x| x.trim().parse::<u32>().expect("error while reading VMAX"))
    .collect();

    if vmax.len() != n {
        panic!("VMAX must be length n");
    }
    if vmin.iter().zip(vmax.iter()).any(|x| x.0 > x.1) {
        panic!("VMIN <= VMAX needed");
    }

    let mut wishes : Vec<Vec<u32>> = Vec::new();
    let mut ids : Vec<String> = Vec::new();

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer).unwrap() == 0 {
            break;
        }
        line_no += 1;

        let mut line : Vec<String> = buffer.split(delimiter).map(|x| x.trim().to_string())/*.filter(|x| !x.is_empty())*/.collect();

        if line.len() == n + 1 {
            ids.push(line.remove(0));
        } else if line.len() == n {
            ids.push(String::new());
        } else {
            print!("\x1B[31m"); // red background green
            println!("l{}:{:?} has been ignored.", line_no, line);
            print!("\x1B[0m"); // reset color
            continue;
        }

        let numbers : Vec<u32> = line.iter()
            .map(|x| x.parse::<u32>().unwrap_or(0))
            .collect();

        wishes.push(numbers.clone());
        let mut copy = numbers.clone();
        copy.sort();
        for i in 0..n {
            if copy[i] != i as u32 {
                print!("\x1B[31m"); // red background green
                println!("l{}:{:?}{:?} does not contain a permutation.", line_no, ids.last().unwrap(), numbers);
                print!("\x1B[0m"); // reset color
                break;
            }
        }
    }

    (vmin, vmax, wishes, ids)
}
