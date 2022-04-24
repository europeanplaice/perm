
#[derive(Debug, Clone)]
struct Node {
    elems: Vec<Vec<NodeOrLeaves>>
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
        $v.borrow_mut().elems.push(vec![NodeOrLeaves::Literal($y.to_string())]);
    }
}

macro_rules! process_nonterminal {
    ($x:ident, |) => {$x.borrow_mut().elems.push(vec![])};
    ($x:ident, +) => {$x.borrow_mut().elems.push(vec![])};
    ($x:ident, $y:literal) => {
        let leaf = NodeOrLeaves::Literal($y.to_string());
        let length = $x.borrow_mut().elems.len();
        $x.borrow_mut().elems[length - 1].push(leaf);
    };
    ($x:ident, $y:ident) => {
        let node : NodeOrLeaves = NodeOrLeaves::Node($y.clone());
        let length = $x.borrow_mut().elems.len();
        $x.borrow_mut().elems[length - 1].push(node);
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
    let expression: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {elems: vec![vec![]]}));
    let number: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {elems: vec![vec![]]}));
    let digit: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {elems: vec![vec![]]}));
    parse_oneline!(expression <- number "+" number | number "-" number);
    parse_oneline!(number <- digit+);
    parse_oneline!(digit <- "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9");
    println!("{:?}", expression);
    println!("{:?}", number);
    println!("{:?}", digit);
}
