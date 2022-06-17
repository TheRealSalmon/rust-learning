pub fn meep() {
    println!("meep")
}

#[derive(Debug, Default)]
pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }

    pub fn traverse_depth(&self) -> Vec<&Node> {
        let parent = self.nodes.get(0).unwrap();
        let mut nodes: Vec<&Node> = vec![parent];

        let mut child: Option<&Node> = Option::None;
        let mut siblings: Vec<&Node> = vec![];

        dbg!(&parent);

        for (i, edge) in parent.edges.iter().enumerate() {
            siblings.push(self.nodes.get(edge.1).unwrap());
            
        }
        let mut counter_1 = 0;
        let mut counter_2 = 0;
        while !siblings.is_empty() && counter_1 < 10 {
            
            child = siblings.pop();

            while child.is_some() && counter_2 < 10 {
                let child_some = child.unwrap();
                nodes.push(child_some);

                if child_some.edges.is_empty() {
                    child = Option::None;
                } else {
                    for (i, edge) in child_some.edges.iter().enumerate() {
                        if i == 0 {
                            child = Some(self.nodes.get(edge.1).unwrap());
                        } else {
                            siblings.push(self.nodes.get(edge.1).unwrap());
                        }
                    }
                }

                counter_2 += 1;
            }
            counter_1 += 1;
        }

        nodes
    }

    pub fn traverse_breadth(&self) -> Vec<&Node> {        
        let parent = self.nodes.get(0).unwrap();
        let mut nodes: Vec<&Node> = vec![parent];

        let mut children: Vec<&Node> = vec![];
        for edge in &parent.edges {
            children.push(self.nodes.get(edge.1).unwrap())
        }

        let mut counter = 0;
        while !children.is_empty() && counter < 10 {
            dbg!(&counter);
            dbg!(&children);

            let mut new_children: Vec<&Node> = vec![];
            for child in children {
                for edge in &child.edges {
                    new_children.push(self.nodes.get(edge.1).unwrap());
                }
                nodes.push(child);
            };
            children = new_children;
            counter += 1;
        }

        nodes
    }
}

// #[derive(Debug, Default)]
// pub struct Graph {
//     nodes: Vec<Node>,
//     edges: Vec<Edge>,
// }

// impl Graph {
//     pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
//         Self { nodes, edges }
//     }
// }

#[derive(Debug, Default)]
pub struct Node {
    pub index: usize,
    pub edges: Vec<(usize, usize)>,
}

impl Node {
    pub fn new(index: usize, edges: Vec<(usize, usize)>) -> Self {
        Self { index, edges } 
    }
}

// #[derive(Debug, Default)]
// pub struct Edge {
//     pub index_tuple: (usize, usize),
// }

// impl Edge {
//     pub fn new(index_tuple: (usize, usize)) -> Self { Self { index_tuple } }
// }