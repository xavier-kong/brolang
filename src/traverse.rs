use crate::Node;

pub fn traverse(node: Node) {
    println!("{:?}", node.data.slice);

    let nodes: Vec<Node> = vec![node];

    while nodes.len() != 0 {
        let new_nodes: Vec<Node> = vec![];

        for (i, n) in nodes.iter().enumerate() {
            println!("{:?}", n);
            if n.left.is_some() {
                match n.left {
                    Some(val) => new_nodes.push(*val),
                    None => ()
                }
            }
            if n.right.is_some()  {
                match n.right {
                    Some(val) => new_nodes.push(*val),
                    None => ()
                }
            }
        }
    }
}
