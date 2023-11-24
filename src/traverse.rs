use crate::Node;

pub fn traverse(node: Node) {
    println!("{:?}", node.data.slice);

    let mut nodes: Vec<&Node> = vec![&node];

    while nodes.len() != 0 {
        let mut new_nodes: Vec<&Node> = vec![];

        for (_i, n) in nodes.iter().enumerate() {
            println!("{:?}", n);

            match &n.left {
                Some(val) => new_nodes.push(val),
                None => ()
            }
            match &n.right {
                Some(val) => new_nodes.push(val),
                None => ()
            }
        }

        nodes = new_nodes;
        println!("{:?}", nodes);
    }

    println!("done");
}
