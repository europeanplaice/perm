
#[derive(Debug, Clone)]
struct Node {
    children: Vec<NodeOrLeaves>,
    brother: Option<NodeOrLeaves>
}

#[derive(Debug, Clone)]
struct Leaf {
    brother: Option<Box<NodeOrLeaves>>,
    literal: String,
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
enum NodeOrLeaves {
    Node(Rc<RefCell<Node>>),
    Leaf(Leaf),
}

impl NodeOrLeaves {
    pub fn set_brother(&mut self, brother: NodeOrLeaves) -> (){
        match self {
            NodeOrLeaves::Node(n) => {
                let temp2;
                let temp = n.borrow();
                match temp.brother{
                    None => {n.borrow_mut().brother = Some(brother.clone())},
                    Some(_) => {temp2 = temp.brother.clone()}
                }
                n.borrow_mut().brother = Some(brother.clone())
            },
            NodeOrLeaves::Leaf(l) => (l.brother = Some(Box::new(brother))),
        }
    }
}
macro_rules! process_terminal{
    ($v:ident, $y:literal) => {
        $v.borrow_mut().elems.push(vec![NodeOrLeaves::Literal($y.to_string())]);
    }
}

macro_rules! process_nonterminal {
    ($y:ident, /) => {$y.borrow_mut().children.push(NodeOrLeaves::Leaf(Leaf{literal: "".to_string(), brother: None}))};
    ($y:ident, $x:literal) => {
        let leaf = NodeOrLeaves::Leaf(Leaf{literal: $x.to_string(), brother: None});
        let current_children_length = $y.borrow_mut().children.len();
        if current_children_length == 0 {
            $y.borrow_mut().children.push(leaf);
        } else {
            $y.borrow_mut().children[current_children_length - 1].set_brother(leaf);
        }
    };
    ($y:ident, $x:ident) => {
        let node : NodeOrLeaves = NodeOrLeaves::Node($x.clone());
        let current_children_length = $y.borrow_mut().children.len();
        if current_children_length == 0 {
            $y.borrow_mut().children.push(node);
        } else {
            $y.borrow_mut().children[current_children_length - 1].set_brother(node);
        }
    };
    ($y:ident, ($($x:tt)*)) => {
        $(
            process_nonterminal!($y, $x);
        )*
    };
}

macro_rules! parse_oneline {
    ($y:ident <- $($x:tt)*) => (
        $(
             process_nonterminal!($y, $x);
        )*
    );
    
}

fn main() {
    let expression: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {children: vec![], brother: None}));
    let number: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {children: vec![], brother: None}));
    let digit: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {children: vec![], brother: None}));
    // parse_oneline!(expression <- number "+" number / number "-" number);
    // parse_oneline!(number <- digit+);
    parse_oneline!(number <- "1" "3" "9" / "2" "4");
    println!("{:?}", number);
    // println!("{:?}", number);
    // println!("{:?}", digit);
}
