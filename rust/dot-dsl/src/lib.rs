macro_rules! with_attrs {
    () => {
        pub fn with_attrs(mut self, _attrs: &[(&str, &str)]) -> Self {
            for (key, value) in _attrs {
                self.attrs.insert(key.to_string(), value.to_string());
            }
            self
        }
    };
}

pub mod graph {
    use std::collections::HashMap;

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, _nodes: &Vec<Node>) -> Self {
            self.nodes.extend_from_slice(_nodes);
            self
        }
        pub fn with_edges(mut self, _edges: &Vec<Edge>) -> Self {
            self.edges.extend_from_slice(_edges);
            self
        }

        with_attrs!();

        pub fn get_node(&self, name: &str) -> Result<&Node, &'static str> {
            let node = self.nodes.iter().find(|&n| n.name == name);
            if node.is_none() {
                Err("can't find node")
            } else {
                Ok(node.unwrap())
            }
        }
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                with_attrs!();

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    let attr = self.attrs.iter().find(|(k, _)| k.eq(&key));
                    if attr.is_none() {
                        None
                    } else {
                        Some(attr.unwrap().1)
                    }
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                begin: String,
                end: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(begin: &str, end: &str) -> Self {
                    Edge {
                        begin: begin.to_string(),
                        end: end.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                with_attrs!();
            }
        }
    }
}
