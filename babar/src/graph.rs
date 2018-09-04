#![allow(dead_code)]

/// The dependency graph.
/// We insert a formula and check if another formula appears in its range.
/// If that is the case, we add this formula as a daughter of the first one in the graph.
/// We then do the same thing for the daughter.
///
/// Note that we intended to use a color code to mark the formulas as we built the graph,
/// but we are still not sure on the proper time to do it. See the comment in the main for
/// more details.

use cell;

/// The colors used to detect cycles :
/// White: this formula has not been evaluated yet
/// Black: this formula has been evaluated and returned aresult
/// Grey: this formula is currently being evaluated
/// Red: this formula has been evaluated and is part of a cycle or not properly formatted
pub enum Color {
    White,
    Black,
    Grey,
    Red,
}

/// The structure representing the nodes in the graph.
pub struct Node<'a> {
    pub value: Box<&'a cell::Formula>,
    pub c: Color,
    pub child_list: Vec<Node<'a>>,
}

/// Parses the graph from the Node node.
fn evaluate(mut node: Node) {
    match node.c {
        Color::White => {
            node.c = Color::Grey;
            for mut n in node.child_list {
                match n.c {
                    Color::White => {
                        println!("child white, evaluate(child)");
                        evaluate(n);
                    }
                    Color::Black => {
                        // TODO : get value and do the appropriate tests with it
                        println!("child black");
                    }
                    _ => {
                        node.c = Color::Red;
                        println!("child red or grey, this node goes red");
                    }
                }
            }
        }
        _ => println!("this node is red or black -> we do nothing"),
    }

    // Evaluates the Node if not already done (ie it was not Balck or White).
    match node.c {
        Color::Grey | Color::White => {
            // TODO: load the memory zone the formula applies to, since the Node is not Red
            println!("evaluate this node value");
            node.c = Color::Black;
        }
        Color::Red => println!("this node value = P"),

        // Should never be accessed
        _ => println!("already evaluated"),
    }
}
