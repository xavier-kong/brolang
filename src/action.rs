use crate::{Node, Token};

enum OperationType {
    Equals,
    Add
}

enum ReturnType {
    Int,
    String,
    Var
}

struct ActionTreeNode {
    operation_type: OperationType,
    return_type: ReturnType,
    arg_left: Option<Box<ActionTreeNode>>,
    arg_right: Option<Box<ActionTreeNode>>,
}

pub fn parse(node: &Node) -> ActionTreeNode {
    let curr_node = *node;



    println!("{:?}", node);
}
