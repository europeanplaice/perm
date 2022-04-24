
#[derive(Debug, Clone)]
struct Node {
    elems: Vec<NodeOrLeaves>
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
enum NodeOrLeaves {
    Node(Rc<RefCell<Node>>),
    Literal(String),
}
macro_rules! process_terminal{
    ($v:ident, $y:literal) => {
        $v.borrow_mut().elems.push(NodeOrLeaves::Literal($y.to_string()));
    }
}

macro_rules! process_nonterminal {
    ($x:ident, |) => {};
    ($x:ident, +) => {};
    ($x:ident, $y:literal) => {
        let leaf = NodeOrLeaves::Literal($y.to_string());
        $x.borrow_mut().elems.push(leaf);
    };
    ($x:ident, $y:ident) => {
        let node : NodeOrLeaves = NodeOrLeaves::Node($y.clone());
        $x.borrow_mut().elems.push(node);
    };
    ($x:ident, ($($y:tt)*)) => {
        $(
            process_nonterminal!($x, $y);
        )*
    };
}

macro_rules! parse_oneline {
    ($x:ident <- $($y:literal)|*) => (
        $(
            process_terminal!($x, $y);
        )*
    );
    ($x:ident <- $($y:tt)*) => (
        $(
             process_nonterminal!($x, $y);
        )*
    );
    
}

fn main() {
    let expression: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {elems: vec![]}));
    let number: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {elems: vec![]}));
    let digit: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {elems: vec![]}));
    parse_oneline!(expression <- number "+" number | number "-" number);
    parse_oneline!(number <- digit+);
    parse_oneline!(digit <- "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9");
    println!("{:?}", expression);
    println!("{:?}", number);
    println!("{:?}", digit);
}
