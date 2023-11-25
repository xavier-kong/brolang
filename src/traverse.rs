use crate::Node;

pub fn traverse(node: Node) {
    println!("{:?}", node.data.slice);

    let mut nodes: Vec<Option<Box<Node>>> = vec![Some(Box::new(node))];

    while nodes.len() != 0 {
        let mut new_nodes: Vec<Option<Box<Node>>> = vec![];

        for (_i, n) in nodes.iter().enumerate() {
            println!("{:?}", n);

            let curr = n;

            let val = match curr {
                Some(val) => {
                    return val;
                    new_nodes.push(val.left);
                    new_nodes.push(val.right);
                },
                None => new_nodes.push(None)
            };
        }

        nodes = new_nodes;
        println!("{:?}", nodes);
    }

    println!("done");
}
