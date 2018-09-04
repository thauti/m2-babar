/// Processes the data extracted with parser.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use cell;
use parser;

/// Checks if the cell is within the range of the current evaluation.
pub fn is_dependency_ok(cell: &cell::Formula, current_evaluation: &Vec<(i32, i32)>) -> bool {
    for &(row, col) in current_evaluation {
        if row >= cell.r1 && row <= cell.r2 && col >= cell.c1 && col <= cell.c2 {
            return false;
        }
    }
    true
}

/// TODO
pub fn calcul_occ(
    cell: &cell::Formula,
    spreadsheet: &Vec<Vec<Box<cell::Cell>>>,
    current_evaluation: &mut Vec<(i32, i32)>,
    posr: i32,
    posc: i32,
) -> (i32, BTreeMap<(i32, i32), Vec<(i32, i32)>>) {
    let r1 = cell.r1 as usize;
    let r2 = cell.r2 as usize;
    let c1 = cell.c1 as usize;
    let c2 = cell.c2 as usize;
    let mut dependencies = BTreeMap::new();
    let mut val_num = 0;
    for i in r1..r2 + 1 {
        for j in c1..c2 + 1 {
            let (val, _) =
                spreadsheet[i][j].evaluate(spreadsheet, current_evaluation, i as i32, j as i32);
            if val < 0 {
                return (val, BTreeMap::new());
            }
            dependencies.insert((i as i32, j as i32), Vec::new());

            match dependencies.get_mut(&(i as i32, j as i32)) {
                Some(dependency) => dependency.push((posr, posc)),
                _ => println!("err"),
            }
            if val == cell.val {
                val_num = val_num + 1;
            }
        }
    }
    (val_num, dependencies)
}

// /!\ WARNING /!\
// NOT CURRENTLY USED, BUT WILL BE SOON™
/// Evaluates all the Formula in the list of formulas.
#[allow(dead_code)]
pub fn evaluate(
    spreadsheet: &Vec<Vec<Box<cell::Cell>>>,
) -> (
    Vec<Vec<Box<cell::Cell>>>,
    BTreeMap<(i32, i32), Vec<(i32, i32)>>,
) {
    let mut new_grid: Vec<Vec<Box<cell::Cell>>> = Vec::new();
    let mut dependencies: BTreeMap<(i32, i32), Vec<(i32, i32)>> = BTreeMap::new();
    for i in 0..spreadsheet.len() {
        let mut row: Vec<Box<cell::Cell>> = Vec::new();
        for j in 0..spreadsheet[i].len() {
            let mut cell = spreadsheet[i][j].copy_cell();
            let mut current_evaluation: Vec<(i32, i32)> = Vec::new();
            let (val, d) = cell.evaluate(spreadsheet, &mut current_evaluation, i as i32, j as i32);
            cell.set_value(val);
            row.push(cell);
            for (&key, val) in d.iter() {
                if !dependencies.contains_key(&key) {
                    dependencies.insert(key, Vec::new());
                }
                match dependencies.get_mut(&key) {
                    Some(dependency) => dependency.extend(val),
                    _ => println!("err"),
                }
            }
        }
        new_grid.push(row);
    }
    (new_grid, dependencies)
}

// /!\ WARNING /!\
// NOT CURRENTLY USED, BUT WILL BE SOON™
/// Supposed to handle view0.csv, but somehow does not work very well.
#[allow(dead_code)]
pub fn write_view0(view0: &str, t: &Vec<Vec<Box<cell::Cell>>>) {
    let mut file = File::create(view0).expect("Error writing file");
    let mut mystring = String::new();
    let mut i: i32 = 0;
    for k in t {
        for b in k {
            if i != 0 {
                mystring += ";";
            }
            i += 1;
            let tmp = b.get_string_value();
            mystring.push_str(&tmp);
        }
        i = 0;
        mystring += "\n";
    }
    write!(file, "{}", mystring).expect("Error Writing into the view0");
}

// /!\ WARNING /!\
// NOT CURRENTLY USED, BUT WILL BE SOON™
/// Supposed to handle changes.txt, but we are VERY far from that, so juste ignore that
#[allow(dead_code)]
pub fn write_change(
    user: &str,
    change: &str,
    spreadsheet: &mut Vec<Vec<Box<cell::Cell>>>,
    dependencies: &mut BTreeMap<(i32, i32), Vec<(i32, i32)>>,
) {
    let mut file_change = File::create(change).expect("Error at file creation");
    let file_user = File::open(user).expect("Error at file opening");
    for line in BufReader::new(file_user).lines() {
        let mut line_iter = line.expect("Error write_change");
        let mut ite = line_iter.trim().split_whitespace();
        let r = match ite.next() {
            Some(x) => x,
            None => continue,
        };
        let c = match ite.next() {
            Some(x) => x,
            None => continue,
        };
        let d = match ite.next() {
            Some(x) => x,
            None => continue,
        };
        let r: i32 = r.trim().parse().expect("bad format");
        let c: i32 = c.trim().parse().expect("bad format");
        let mut current_evaluation: Vec<(i32, i32)> = Vec::new();
        let mut cell = parser::create_cell(d.to_string());
        let (val, de) = cell.evaluate(spreadsheet, &mut current_evaluation, r, c);
        cell.set_value(val);
        let (v, row1, row2, col1, col2) = cell.get_fields();
        if spreadsheet[r as usize][c as usize].is_same_cell(v, row1, row2, col1, col2) {
            continue;
        }
        let (r1, r2, c1, c2) = spreadsheet[r as usize][c as usize].get_region();
        if (r1, r2, c1, c2) != (-1, -1, -1, -1) {
            for i in r1..r2 {
                for j in c1..c2 {
                    match dependencies.get_mut(&(i as i32, j as i32)) {
                        Some(dependence) => dependence.retain(|&x| x != (r, c)),
                        _ => println!("err"),
                    }
                }
            }
        }
        for (&key, val) in de.iter() {
            if !dependencies.contains_key(&key) {
                dependencies.insert(key, Vec::new());
            }
            match dependencies.get_mut(&key) {
                Some(dependency) => dependency.extend(val),
                _ => (),
            }
        }
        spreadsheet[r as usize][c as usize] = cell;
        write!(file_change, "after  \"{} {} {}\":\n", r, c, d)
            .expect("Error Writing into the change");
        match dependencies.get(&(r, c)) {
            Some(d) => for &(x, y) in d {
                let mut cell = spreadsheet[x as usize][y as usize].copy_cell();
                current_evaluation.clear();
                let (val, _) = cell.evaluate(spreadsheet, &mut current_evaluation, x, y);
                if val != spreadsheet[x as usize][y as usize].get_value() {
                    cell.set_value(val);
                    spreadsheet[x as usize][y as usize] = cell;
                    write!(file_change, "{} {} {}\n", x, y, val)
                        .expect("Error Writing into the change");
                }
            },
            _ => (),
        }
    }
}
