use std::{borrow::Cow, cmp::max, io, rc::Rc};

use ptree::{Style, TreeBuilder, TreeItem, item::StringItem};

#[derive(Debug, Default, Clone)]
pub struct Node {
    weight: usize,
    str_content: Option<Rc<str>>,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    depth: usize,
    length: usize,
}


impl Node {
    pub fn new(str_content: String) -> Self {
        let length = str_content.chars().count();
        Self {
            weight: length,
            str_content: Some(str_content.into()),
            left: None,
            right: None,
            depth: 0,
            length,
        }
    }
    fn is_balanced(&self) -> bool {
        self.length >= FIBONACCI[self.depth + 2]
    }

    pub fn right(&self) -> Option<&Node> {
        self.right.as_deref()
    }
    pub fn pretty_print(&self)->StringItem{
        
        match self.str_content{
            Some(ref content) => {
                let node_details=format!("leaf :'{}' weight:{} depth:{}",content,self.weight,self.depth);
                let mut tree=TreeBuilder::new(node_details);
                tree.build()

            },
            None => {
                let node_details=format!("root weight:{} depth:{}",self.weight,self.depth);
                let mut tree=TreeBuilder::new(node_details);
                if let Some(ref left_child) = self.left {
                    left_child.print_my_tree(&mut tree); 
                } 
                if let Some(ref right_child) = self.right {
                    right_child.print_my_tree(&mut tree);
                } 
                tree.build()
            }
        }
    }
    pub fn print_my_tree(&self,print_tree:&mut TreeBuilder){
        match self.str_content{
            Some(ref content) => {
                let node_details=format!("leaf :'{}' weight:{} depth:{}",content,self.weight,self.depth);
                print_tree.add_empty_child(node_details);   
            },
            None => {
                let node_details=format!("internal weight:{} depth:{}",self.weight,self.depth);
                let print_tree=print_tree.begin_child(node_details);
                if let Some(ref left_child) = self.left {
                    left_child.print_my_tree(print_tree); 
                } 
                if let Some(ref right_child) = self.right {
                    right_child.print_my_tree(print_tree);
                } 
                print_tree.end_child();  
            }
        };
        
        
        
    }
}

impl TreeItem for Node {
    type Child = Self;
    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        match self.str_content{
            Some(ref content) => {
                let node_details=format!("leaf :'{}' weight:{} depth:{}",content,self.weight,self.depth);
                write!(f, "{}", style.paint(node_details))
                
                
                
            },
            None => {
                let node_details=format!("internal weight:{} depth:{}",self.weight,self.depth);
                write!(f, "{}", style.paint(node_details))
                
            },
        }
    }
    fn children(&self) -> Cow<'_,[Self::Child]> {
        let mut children=Vec::new();
        if let Some(ref left_child) = self.left {
            children.push(left_child.as_ref().clone()); 
        } 
        if let Some(ref right_child) = self.right {
            children.push(right_child.as_ref().clone()); 
        } 
        Cow::from(children)
    }
}

const LEAF_LEN: usize = 3;
const FIBONACCI: [usize; 30] = [
     0,1,1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
    10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229,
];

pub fn build_rope(content: &[char], starting: usize, ending: usize) -> (Box<Node>, usize) {
    if ending - starting >= LEAF_LEN {
        let mut rope = Node::default();
        let mid_point = (starting + ending) / 2;
        let (left, left_depth) = build_rope(content, starting, mid_point);
        let (right, right_depth) = build_rope(content, mid_point + 1, ending);
        let depth = 1 + max(left_depth, right_depth);
        rope.length = (ending - starting) + 1;
        rope.weight = (mid_point - starting) + 1;
        rope.left = Some(left);
        rope.right = Some(right);
        rope.depth = depth;
        (Box::new(rope), depth)
    } else {
        let str_content: String = content[starting..=ending].iter().collect();
        (Box::new(Node::new(str_content)), 0)
    }
}
pub fn collect_string(node: &Node, content: &mut String) {
    match node.str_content {
        Some(ref current) => {
            content.push_str(current);
        }
        None => {
            if let Some(ref left_node) = node.left {
                collect_string(left_node, content);
            }

            if let Some(ref right_node) = node.right {
                collect_string(right_node, content);
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

pub fn split(rope: &mut Node, index: usize, cut_nodes: &mut Vec<Box<Node>>) -> (usize, bool, bool) {
    match rope.str_content {
        Some(ref content) => {
            if index == 0 {
                (rope.weight, true, false)
            } else if index < content.chars().count() {
                let full_content: Vec<char> = content.chars().collect();
                let left_content = &full_content[0..index];
                let left_str: String = left_content.iter().collect();
                let right_content = &full_content[index..];
                let right_str = right_content.iter().collect();
                let left = Node::new(left_str);
                let right = Node::new(right_str);
                cut_nodes.push(Box::new(right));
                let parent = Node{
                    weight: left_content.len(),
                    left: Some(Box::new(left)),
                    depth: 1,
                    length: left_content.len(),
                    ..Default::default()
                };
                *rope = parent;
                (right_content.len(), false, true)
            } else {
                (0, false, false)
            }
        }
        None => {
            if index < rope.weight {
                match rope.left {
                    Some(ref mut left) => {
                        let (weight_to_reduce, should_delete_child, should_increase_depth) =
                            split(left, index, cut_nodes);
                        rope.weight -= weight_to_reduce;
                        rope.length -= weight_to_reduce;
                        if should_increase_depth {
                            rope.depth = max(rope.depth, left.depth + 1);
                        }
                        if should_delete_child {
                            let left = rope.left.take();
                            rope.depth = 0;
                            if let Some(cut_node) = left {
                                cut_nodes.push(cut_node);
                            }
                        }
                        let right = rope.right.take();
                        if let Some(cut_node) = right {
                            cut_nodes.push(cut_node);
                        }
                        (weight_to_reduce, false, should_increase_depth)
                    }
                    None => (0, false, false),
                }
            } else {
                match rope.right {
                    Some(ref mut right) => {
                        let (weight_to_reduce, should_delete_child, should_increase_depth) =
                            split(right, index - rope.weight, cut_nodes);
                        rope.length -= weight_to_reduce;
                        if should_increase_depth {
                            rope.depth = max(rope.depth, right.depth);
                        }
                        if should_delete_child {
                            let right = rope.right.take();
                            if let Some(cut_node) = right {
                                cut_nodes.push(cut_node);
                            }
                        }

                        (weight_to_reduce, false, should_increase_depth)
                    }
                    None => (0, false, false),
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

pub fn concatenate(left: Box<Node>, right: Box<Node>) -> Node {
    // if let Some(ref left_str_content) = left.str_content {
    //     if let Some(ref right_str_content) = right.str_content {
    //         let left_count = left_str_content.chars().count();
    //         let right_count = right_str_content.chars().count();

    //         if left_count + right_count <= LEAF_LEN {
    //             let combined_string: String = format!("{}{}", left_str_content, right_str_content);
    //             return Node::new(combined_string);
    //         }
    //     }
    // }
    // if let Some(ref left_right_child) = left.right {
    //     if let Some(ref str_content) = left_right_child.str_content {
    //         if let Some(ref right_str_content) = right.str_content {
    //             let left_count = str_content.chars().count();
    //             let right_count = right_str_content.chars().count();

    //             if left_count + right_count <= LEAF_LEN {
    //                 let combined_string: String = format!("{}{}", str_content, right_str_content);
    //                 let new_left_right_child = Node::new(combined_string);
    //                 let mut new_node = Node::default();
    //                 new_node.depth = left.depth;
    //                 new_node.length = left.length + right_count;
    //                 new_node.left = left.left;
    //                 new_node.weight = left.weight;
    //                 new_node.right = Some(Box::new(new_left_right_child));
    //                 return new_node;
    //             }
    //         }
    //     }
    // }
    Node { depth: 1 + max(left.depth, right.depth), length: left.length + right.length, weight: left.length, left: Some(left), right: Some(right), ..Default::default() }
}

pub fn insert(index: usize, rope: Box<Node>, content: String) -> Node {
    // println!("rope before {:?}",rope);
    let mut original_rope = rope;
    let mut cut_nodes = Vec::new();

    let _ = split(&mut original_rope, index, &mut cut_nodes);
    // println!("cut is {}",cut.0);

    let new_merged_cut_nodes = {
        let content: Vec<char> = content.chars().collect();
        // println!("content len is {}",content.len());
        let (mut merged, _) = build_rope(&content, 0, content.len() - 1);
        for cut_node in cut_nodes {
            merged = Box::new(concatenate(merged, cut_node));
        }
        merged
    };

    concatenate(original_rope, new_merged_cut_nodes)
}

pub fn remove(index: usize, rope: Box<Node>, length_to_cut: usize) -> Box<Node> {
    let mut original_rope = rope;
    let mut cut_nodes = Vec::new();

    let _ = split(&mut original_rope, index, &mut cut_nodes);

    if cut_nodes.is_empty() {
        return original_rope;
    }
    println!("cut nodes is {}", cut_nodes.len());

    let mut new_merged_cut_nodes = {
        let mut cut_nodes = cut_nodes.into_iter();
        let first = cut_nodes.next();
        let mut merged = {
            match first {
                Some(first_cut) => {
                    println!("first cut {:?}", first_cut);
                    first_cut
                }
                None => {
                    return original_rope;
                }
            }
        };
        for cut_node in cut_nodes {
            println!("cut node {:?}", cut_node);
            merged = Box::new(concatenate(merged, cut_node));
        }
        merged
    };

    let mut cut_nodes = Vec::new();
    let _ = split(&mut new_merged_cut_nodes, length_to_cut, &mut cut_nodes);

    if cut_nodes.is_empty() {
        return original_rope;
    }

    let third_new_merged_cut_nodes = {
        let mut cut_nodes = cut_nodes.into_iter();
        let first = cut_nodes.next();
        let mut merged = {
            match first {
                Some(first_cut) => first_cut,
                None => {
                    return original_rope;
                }
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
pub fn collect_leaves(node: Box<Node>, leaves: &mut Vec<Box<Node>>) {
    if node.str_content.is_some() {
        leaves.push(node);
    } else {
        if let Some(left_node) = node.left {
            collect_leaves(left_node, leaves);
        }

        if let Some(right_node) = node.right {
            collect_leaves(right_node, leaves);
        }
    }
}

pub fn rebalance(node: Box<Node>) -> Box<Node> {
    let mut slots: Vec<Option<Box<Node>>> = vec![None; 30];
    let mut leaves = Vec::new();
    collect_leaves(node, &mut leaves);
    'outer:for leaf in leaves {
        let slot_index = match FIBONACCI.binary_search(&leaf.length) {
            Ok(index) => index,
            Err(0) => 0,
            Err(i) => i - 1,
        };
        let mut nodes_to_concatenate=Vec::new();
        
        for i in 0..slot_index{
            if slots[i].is_some(){
                nodes_to_concatenate.push(slots[i].take().unwrap());
            }  
        }
        if nodes_to_concatenate.is_empty(){
            match slots[slot_index].take(){
                Some(current_node) => {
                    let mut merged=leaf;
                    for i in slot_index..slots.len(){
                        let current=slots[i].take();
                        match current{
                            Some(current_node) =>{
                                merged=Box::new(concatenate(current_node, merged));
                                let new_slot_index = match FIBONACCI.binary_search(&merged.length) {
                                    Ok(index) => index,
                                    Err(0) => 0,
                                    Err(i) => i - 1,
                                };
                                if new_slot_index==i{
                                    slots[i]=Some(merged);
                                    continue 'outer;
                                    
                                }
                            },
                            None => {
                                let new_slot_index = match FIBONACCI.binary_search(&merged.length) {
                                    Ok(index) => index,
                                    Err(0) => 0,
                                    Err(i) => i - 1,
                                };
                                if new_slot_index==i{
                                    slots[i]=Some(merged);
                                    continue 'outer;
                                    
                                }
                                
                            },
                        }
                    }
                    
                },
                None => {
                    slots[slot_index]=Some(leaf);
                    continue 'outer;
                },
            }
            
        }else{
            let mut nodes_to_concatenate=nodes_to_concatenate.into_iter().rev();
            let mut merged=nodes_to_concatenate.next().unwrap();
            for node in nodes_to_concatenate{
                merged=Box::new(concatenate(node, merged));
            }
            merged=Box::new(concatenate(merged, leaf));
            for i in slot_index..slots.len(){
                let current=slots[i].take();
                match current{
                    Some(current_node) =>{
                        merged=Box::new(concatenate(current_node, merged));
                        let new_slot_index = match FIBONACCI.binary_search(&merged.length) {
                            Ok(index) => index,
                            Err(0) => 0,
                            Err(i) => i - 1,
                        };
                        if new_slot_index==i{
                            slots[i]=Some(merged);
                            continue 'outer;
                            
                        }
                    },
                    None => {
                        let new_slot_index = match FIBONACCI.binary_search(&merged.length) {
                            Ok(index) => index,
                            Err(0) => 0,
                            Err(i) => i - 1,
                        };
                        if new_slot_index==i{
                            slots[i]=Some(merged);
                            continue 'outer;
                            
                        }
                        
                    },
                }
            }
        }
    }
    let mut result: Option<Box<Node>> = None;
    for slot in slots.into_iter().flatten() {
        result = Some(match result {
            None => slot,
            Some(r) => Box::new(concatenate(r, slot)),
        });
    }
    result.unwrap()
}
