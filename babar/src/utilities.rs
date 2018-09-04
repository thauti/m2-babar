use std;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::str;
use cell;

pub const BUFF_SIZE: usize = 16384;

/// Parses the file line by line and puts every formula in a list.
pub fn read_first_time(path: &str, formulas: &mut Vec<cell::Formula>) {
    let file = File::open(path).expect("fail to open");
    let mut buff = Vec::with_capacity(BUFF_SIZE);
    let mut reader = std::io::BufReader::new(file);
    reader
        .read_until(b'=', &mut buff)
        .expect("read until formula");
    buff.clear();
    let mut _num_bytes = reader.read_until(b')', &mut buff).expect("read formula");
    while _num_bytes != 0
    //Buffer not empty0
    {
        let mut formula: Vec<u8> = Vec::new();
        formula.push(b'=');
        for byte in &buff {
            if byte == (&b';') {
                break;
            } else {
                formula.push(*byte);
            }
        }
        formulas.push(create_formula(String::from_utf8(formula).unwrap()));
        _num_bytes = reader
            .read_until(b'=', &mut buff)
            .expect("read until formula or end file");
        buff.clear();
        _num_bytes = reader.read_until(b')', &mut buff).expect("read file");
    }
}

// /!\ WARNING /!\
// NOT CURRENTLY USED, BUT WILL BE SOON™
/// Writes into view0.csv.
#[allow(dead_code)]
pub fn write_view(path: &str, formulas: &mut Vec<String>) {
    let mut count: i32 = 0;
    let mut view = File::create("view0.csv").expect("Error creating file");
    let file = File::open(path).expect("fail to open");
    let mut buff = BufReader::with_capacity(BUFF_SIZE, file);

    //Checks if formula list is empty
    let mut f = match formulas.pop() {
        Some(x) => x,
        None => return,
    };

    loop {
        let length = {
            let mut buffer = buff.fill_buf().expect("err read_first_time");
            let mut line = String::new();
            let num_byte = buffer.read_line(&mut line).expect("err");
            while line.contains(&f) {
                line = line.replace(&f, "-1");
                f = match formulas.pop() {
                    Some(x) => x,
                    None => break,
                };
            }
            write!(view, "{}", line).expect("Error Writing into the view0");
            num_byte
        };
        count = count + 1;
        if length == 0 {
            break;
        }
        buff.consume(length);
    }
}

/// Extracts the data used to create a Formula from a raw string
pub fn create_formula(form_string: String) -> cell::Formula {
    let form: String = form_string
        .trim_matches(|c| c == '(' || c == ')' || c == '=' || c == '#')
        .to_string();
    let form_decompose = form.split(",");
    let form_dec_vec: Vec<&str> = form_decompose.collect();
    if form_dec_vec.len() < 5 {
        panic!("Erreur format");
    }

    /// Check the cell.rs file for the parameters.
    let formula = cell::Formula {
        num: 0,
        r1: form_dec_vec[0].trim().parse().expect("Erreur format"),
        c1: form_dec_vec[1].trim().parse().expect("Erreur format"),
        r2: form_dec_vec[2].trim().parse().expect("Erreur format"),
        c2: form_dec_vec[3].trim().parse().expect("Erreur format"),

        val: form_dec_vec[4].trim().parse().expect("Erreur format"),
        str_form: form_string,
    };
    return formula;
}

/*
pub fn create_graph(path:&str,formulas:&mut Vec<cell::Formula>){
    let mut tree = searchTree::NodeST{
        value: &formulas[0].str_form.clone(),
        left: None,
        right:None,
    };

    for i in 1..formulas.len() {
        if tree.insert(&formulas[i].str_form) {
            graph::Node{
            value: Box::new(&formulas[i]),
            c: graph::Color::White,
            child_list: Vec::new(),
            };
            //let mut buff = vec![];
            //get_area(&formulas[i],path,&mut buff);
        }
    }
}
*/

// /!\ WARNING /!\
// NOT CURRENTLY USED, BUT WILL BE SOON™
/// Reads a specific area of the source file.
#[allow(dead_code)]
pub fn get_area(formula: cell::Formula, path: &str, buff_target: &mut Vec<u8>) {
    let file = File::open(path).expect("failed to open file");
    let mut reader = std::io::BufReader::new(file);
    let mut jump = String::new();
    for _i in 0..formula.r1 - 1 {
        let _res = reader.read_line(&mut jump).expect("jumping to r1");
    }
    for _i in 0..formula.c1 - 1 {
        let _res = reader.read_until(b';', buff_target);
    }
    buff_target.clear();
    if formula.r2 != formula.r1 {
        for _i in 0..formula.r2 - formula.r1 {
            let _res = reader.read_until(b'\n', buff_target);
        }
        for _i in 0..formula.c2 - 1 {
            let _res = reader.read_until(b';', buff_target);
        }
    } else {
        for _i in 0..formula.c2 - formula.c1 {
            let _res = reader.read_until(b';', buff_target);
        }
    }
}
