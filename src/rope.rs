use std::rc::Rc;
#[derive(Debug, Default)]
pub struct Node {
    weight: usize,
    str_content: Option<Rc<str>>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(str_content: String) -> Self {
        Self {
            weight: 0,
            str_content: Some(str_content.into()),
            left: None,
            right: None,
        }
    }

    pub fn right(&self) -> Option<&Box<Node>> {
        self.right.as_ref()
    }
}
const LEAF_LEN: usize = 3;

pub fn build_rope(content: &[char], starting: usize, ending: usize) -> Box<Node> {
    let mut rope = Node::default();
    if ending - starting >= LEAF_LEN {
        let mid_point = (starting + ending) / 2;
        let left = build_rope(content, starting, mid_point);
        let right = build_rope(content, mid_point + 1, ending);
        rope.weight = (mid_point - starting) + 1;
        rope.left = Some(left);
        rope.right = Some(right);
    } else {
        let str_content: String = content[starting..=ending].iter().collect();
        let rc_from_string: Rc<str> = str_content.into();
        rope.weight = (ending - starting) + 1;
        rope.str_content = Some(rc_from_string);
    }
    Box::new(rope)
}
pub fn collect_string(node: &Node, content: &mut String) {
    match node.str_content {
        Some(ref current) => {
            content.push_str(&current);
        }
        None => {
            if let Some(ref left_node) = node.left {
                collect_string(&left_node, content);
            }

            if let Some(ref right_node) = node.right {
                collect_string(&right_node, content);
            }
        }
    }
}

pub fn index(node: &Node, index: usize) -> Option<char> {
    let mut index = index;
    let mut current_node = Some(node);
    while let Some(rope_node) = current_node {
        match rope_node.str_content {
            Some(ref content) => {
                let mut characters = content.chars();
                let letter = characters.nth(index);
                return letter;
            }
            None => {
                if index < rope_node.weight {
                    current_node = rope_node.left.as_deref();
                } else {
                    index -= rope_node.weight;
                    let right = rope_node.right.as_deref();
                    current_node = right;
                }
            }
        }
    }
    None
}

pub fn split(rope: &mut Node, index: usize, cut_nodes: &mut Vec<Box<Node>>) -> (usize, bool) {
    match rope.str_content {
        Some(ref content) => {
            if index == 0 {
                return (rope.weight, true);
            } else {
                let full_content: Vec<char> = content.chars().collect();
                let left_content = &full_content[0..index];
                let left_str: String = left_content.iter().collect();
                let right_content = &full_content[index..];
                let right_str = right_content.iter().collect();
                let left = Node::new(left_str);
                let right = Node::new(right_str);
                cut_nodes.push(Box::new(right));
                let mut parent = Node::default();
                parent.left = Some(Box::new(left));
                parent.weight = left_content.len();
                *rope = parent;
                return (right_content.len(), false);
            }
        }
        None => {
            if index < rope.weight {
                match rope.left {
                    Some(ref mut left) => {
                        let (weight_to_reduce, should_delete_child) = split(left, index, cut_nodes);
                        rope.weight -= weight_to_reduce;
                        if should_delete_child {
                            let right = rope.left.take();
                            if let Some(cut_node) = right {
                                cut_nodes.push(cut_node);
                            }
                        }
                        let right = rope.right.take();
                        if let Some(cut_node) = right {
                            cut_nodes.push(cut_node);
                        }
                        (weight_to_reduce, false)
                    }
                    None => (0, false),
                }
            } else {
                match rope.right {
                    Some(ref mut right) => {
                        let (weight_to_reduce, should_delete_child) =
                            split(right, index - rope.weight, cut_nodes);
                        if should_delete_child {
                            let right = rope.right.take();
                            if let Some(cut_node) = right {
                                cut_nodes.push(cut_node);
                            }
                        }

                        (weight_to_reduce, false)
                    }
                    None => (0, false),
                }
            }
        }
    }
}

pub fn find_length(node: &Node) -> usize {
    let left_weight = node.weight;

    match node.right {
        Some(ref right_child) => {
            let right_weight = find_length(right_child);
            left_weight + right_weight
        }
        None => left_weight,
    }
}

fn concatenate(left: Box<Node>, right: Box<Node>) -> Node {
    let mut new_node = Node::default();
    let left_child_length = find_length(&left);
    new_node.left = Some(left);
    new_node.right = Some(right);
    new_node.weight = left_child_length;
    new_node
}

pub fn insert(index: usize, rope: Box<Node>, content: String) -> Node {
    let mut original_rope = rope;
    let mut cut_nodes = Vec::new();

    let _ = split(&mut original_rope, index, &mut cut_nodes);

    let new_merged_cut_nodes = {
        let content:Vec<char>=content.chars().collect();
        println!("content len is {}",content.len());
        let mut merged=build_rope(&content,0,content.len()-1);
        for cut_node in cut_nodes {
            merged = Box::new(concatenate(merged, cut_node));
        }
        merged
    };

    let final_parent = concatenate(original_rope, new_merged_cut_nodes);
    final_parent
}

pub fn remove(index: usize, rope: Box<Node>, length_to_cut: usize) -> Box<Node> {
    let mut original_rope = rope;
    let mut cut_nodes = Vec::new();
    

    let _ = split(&mut original_rope, index, &mut cut_nodes);
    
    if cut_nodes.is_empty(){
        return original_rope;
    }
    println!("cut nodes is {}",cut_nodes.len());

    let mut new_merged_cut_nodes = {
        let mut cut_nodes=cut_nodes.into_iter();
        let first=cut_nodes.next();
        let mut merged = {
            match first{
                Some(first_cut) =>{
                    println!("first cut {:?}",first_cut);
                    first_cut  
                },
                None => {
                    return original_rope;
                },
            }
        };
        for cut_node in cut_nodes {
            println!("cut node {:?}",cut_node);
            merged = Box::new(concatenate(merged, cut_node));
        }
        merged
    };
    
    let mut cut_nodes = Vec::new();
    let _ = split(&mut new_merged_cut_nodes, length_to_cut, &mut cut_nodes);
    
    let third_new_merged_cut_nodes = {
        let mut cut_nodes=cut_nodes.into_iter();
        let first=cut_nodes.next();
        let mut merged = {
            match first{
                Some(first_cut) =>{
                    first_cut  
                },
                None => {
                    return original_rope;
                },
            }
        };
        for cut_node in cut_nodes {
            merged = Box::new(concatenate(merged, cut_node));
        }
        merged
    };

    let final_parent = concatenate(original_rope, third_new_merged_cut_nodes);
    Box::new(final_parent)
}
