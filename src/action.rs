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
    OperationType: OperationType,
    ReturnType: ReturnType,
    ArgLeft: Option<Box<ActionTreeNode>>,
    ArgRight: Option<Box<ActionTreeNode>>,
}
