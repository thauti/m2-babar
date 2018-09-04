// For an explanation on how this project works, see pcomp/fonctionnement_projets/notes_rust.txt

mod parser;
mod processing;
mod cell;
mod graph;
mod utilities;
mod search_tree;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        panic!("Erreur d'arguments, nombre : {}", args.len());
    }

    /* VARIABLE DECLARATIONS */
    let mut formulas: Vec<cell::Formula> = Vec::new();
    /* START READING FILE */
    parser::read_file(&args[1], &mut formulas);
    //println!("nombre de formules: {}", formulas.len());

    /*
    let data = parser::read_file(&args[1]);
    let t = parser::gen_table(data); 
    parser::print_table(&t);
    let (mut grid,mut dependences) = treatment::evaluate(&t);
    treatment::write_view0(&args[3],&grid);
    treatment::write_change(&args[2],&args[4],&mut grid,&mut dependences);
    */
}
