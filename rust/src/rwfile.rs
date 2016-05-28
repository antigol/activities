use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

pub fn write_file(out_file: &String, results: &Vec<usize>) {
    let mut buffer = File::create(out_file).expect("error when open output file");

    for i in 0..results.len() {
        write!(buffer, "{}\n", results[i]).unwrap();
    }
}

pub fn read_file(in_file: &String, delimiter: &String) -> (Vec<u32>, Vec<u32>, Vec<Vec<u32>>) {
    let f = File::open(in_file).expect("error when open file");
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    reader.read_line(&mut buffer).expect("The file is too short");
    let vmin : Vec<u32> = buffer.split(delimiter)
    .map(|x| x.trim())
    .filter(|x| !x.is_empty())
    .map(|x| x.trim().parse::<u32>().expect("error while reading VMIN"))
    .collect();
    let n = vmin.len();

    buffer.clear();
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

    let mut line_no = 3;

    buffer.clear();
    while reader.read_line(&mut buffer).unwrap() > 0 {
        let line : Vec<u32> = buffer.split(delimiter)
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<u32>().expect("error while reading WISHES"))
        .collect();

        if line.len() != n {
            panic!("WHISH must be m by n");
        }
        wishes.push(line.clone());
        let mut copy = line.clone();
        copy.sort();
        for i in 0..n {
            if copy[i] != i as u32 {
                println!("The line number {} ({}th line of WISHES) in the input file does not contain a permutation", line_no, line_no-2);
                break;
            }
        }

        line_no += 1;

        buffer.clear();
    }

    (vmin, vmax, wishes)
}
