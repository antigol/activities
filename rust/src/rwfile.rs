use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

pub fn write_file(out_file: &String, results: &Vec<Vec<usize>>, delimiter: &String) {
    let mut buffer = File::create(out_file).expect("error when open output file");

    for i in 0..results[0].len() {
        for k in 0..results.len() {
            write!(buffer, "{}{}", results[k][i], delimiter).unwrap();
        }
        write!(buffer, "\n").unwrap();
    }
}

pub fn read_file(in_file: &String, delimiter: &String) -> (Vec<u32>, Vec<u32>, Vec<Vec<u32>>) {
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

    buffer.clear();
    while reader.read_line(&mut buffer).unwrap() > 0 {
        line_no += 1;

        let line : Vec<u32> = buffer.split(delimiter)
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse::<u32>().expect(format!("error while reading WISHES at line {}", line_no).as_str()))
            .collect();

        if line.len() != n {
            print!("\x1B[31m"); // red background green
            println!("Line {} has been ignored.", line_no);
            print!("\x1B[0m"); // reset color
            continue;
        }
        wishes.push(line.clone());
        let mut copy = line.clone();
        copy.sort();
        for i in 0..n {
            if copy[i] != i as u32 {
                print!("\x1B[31m"); // red background green
                println!("Line {} does not contain a permutation.", line_no);
                print!("\x1B[0m"); // reset color
                break;
            }
        }

        buffer.clear();
    }

    (vmin, vmax, wishes)
}
