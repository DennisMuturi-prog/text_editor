use std::{
    borrow::Cow,
    cmp::{max, min},
    io,
    ops::Deref,
};

use ptree::{Style, TreeBuilder, TreeItem, item::StringItem, print_tree};
use unicode_segmentation::UnicodeSegmentation;

use crate::rc_substr::RcSubstr;

#[derive(Debug, Default, Clone)]
pub struct Node {
    weight: usize,
    str_content: Option<RcSubstr>,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    depth: usize,
    length: usize,
}

impl Node {
    pub fn new(str_content: String, length: usize) -> Self {
        let rc_str = str_content.into();
        Self {
            weight: length,
            str_content: Some(RcSubstr::new(rc_str)),
            left: None,
            right: None,
            depth: 0,
            length,
        }
    }
    pub fn is_balanced(&self) -> bool {
        self.length >= FIBONACCI[self.depth + 2]
    }

    pub fn right(&self) -> Option<&Node> {
        self.right.as_deref()
    }
    pub fn pretty_print(&self) -> StringItem {
        match self.str_content {
            Some(ref content) => {
                let node_details = format!(
                    "leaf :'{}' weight:{} depth:{}",
                    content.deref(),
                    self.weight,
                    self.depth
                );
                let mut tree = TreeBuilder::new(node_details);
                tree.build()
            }
            None => {
                let node_details = format!("root weight:{} depth:{}", self.weight, self.depth);
                let mut tree = TreeBuilder::new(node_details);
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
    pub fn print_my_tree(&self, print_tree: &mut TreeBuilder) {
        match self.str_content {
            Some(ref content) => {
                let node_details = format!(
                    "leaf :'{}' weight:{} depth:{}",
                    content.deref(),
                    self.weight,
                    self.depth
                );
                print_tree.add_empty_child(node_details);
            }
            None => {
                let node_details = format!("internal weight:{} depth:{}", self.weight, self.depth);
                let print_tree = print_tree.begin_child(node_details);
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

    pub fn remove(self: Box<Self>, index: usize, length_to_cut: usize) -> Box<Node> {
        let mut original_rope = self;
        let mut cut_nodes = Vec::new();

        let _ = split(&mut original_rope, index, &mut cut_nodes);

        let original_rope = rebalance(original_rope);

        if cut_nodes.is_empty() {
            return original_rope;
        }

        let mut new_merged_cut_nodes = {
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
                merged = concatenate(merged, cut_node);
            }
            merged
        };
        new_merged_cut_nodes = rebalance(new_merged_cut_nodes);

        let mut cut_nodes = Vec::new();
        let _ = split(&mut new_merged_cut_nodes, length_to_cut, &mut cut_nodes);

        if cut_nodes.is_empty() {
            return original_rope;
        }

        let mut third_new_merged_cut_nodes = {
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
                merged = concatenate(merged, cut_node);
            }
            merged
        };
        third_new_merged_cut_nodes = rebalance(third_new_merged_cut_nodes);

        concatenate(original_rope, third_new_merged_cut_nodes)
    }
}

impl TreeItem for Node {
    type Child = Self;
    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        match self.str_content {
            Some(ref content) => {
                let node_details = format!(
                    "leaf :'{}' weight:{} depth:{} length:{}",
                    content.deref(),
                    self.weight,
                    self.depth,
                    self.length
                );
                write!(f, "{}", style.paint(node_details))
            }
            None => {
                let node_details = format!(
                    "internal weight:{} depth:{} l:{}",
                    self.weight, self.depth, self.length
                );
                write!(f, "{}", style.paint(node_details))
            }
        }
    }
    fn children(&self) -> Cow<'_, [Self::Child]> {
        let mut children = Vec::new();
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
    0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
    10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229,
];

pub fn build_rope(content: &[&str], starting: usize, ending: usize) -> (Box<Node>, usize) {
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
        let before = content[starting..=ending].iter().copied();
        let str_content: String = before.collect();
        (Box::new(Node::new(str_content, (ending - starting) + 1)), 0)
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

pub fn index(node: &Node, index: usize) -> Option<&str> {
    let mut index = index;
    let mut current_node = Some(node);
    while let Some(rope_node) = current_node {
        match rope_node.str_content {
            Some(ref content) => {
                let mut characters = content.graphemes(true);
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

pub fn split(rope: &mut Node, index: usize, cut_nodes: &mut Vec<Box<Node>>) -> (bool, bool) {
    match rope.str_content {
        Some(ref content) => {
            if index == 0 {
                (true, false)
            } else if index < rope.length {
                let full_content: Vec<&str> = content.graphemes(true).collect();
                let left_content = &full_content[0..index];
                let left_str: String = left_content.iter().copied().collect();
                let right_content = &full_content[index..];
                let right_str = right_content.iter().copied().collect();
                let left = Node::new(left_str, index);
                let right = Node::new(right_str, full_content.len() - index);
                cut_nodes.push(Box::new(right));
                let parent = Node {
                    weight: left_content.len(),
                    left: Some(Box::new(left)),
                    depth: 1,
                    length: left_content.len(),
                    ..Default::default()
                };
                *rope = parent;
                (false, true)
            } else {
                (false, false)
            }
        }
        None => {
            if index < rope.weight {
                match rope.left {
                    Some(ref mut left) => {
                        let (should_delete_child, should_increase_depth) =
                            split(left, index, cut_nodes);
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
                            rope.length -= cut_node.length;
                            cut_nodes.push(cut_node);
                        }
                        rope.weight = {
                            match rope.left {
                                Some(ref left_child) => left_child.length,
                                None => 0,
                            }
                        };
                        rope.length = {
                            let left_len = match rope.left {
                                Some(ref left_child) => left_child.length,
                                None => 0,
                            };
                            let right_len = match rope.right {
                                Some(ref right_child) => right_child.length,
                                None => 0,
                            };
                            right_len + left_len
                        };
                        (rope.length == 0, should_increase_depth)
                    }
                    None => (false, false),
                }
            } else {
                match rope.right {
                    Some(ref mut right) => {
                        let (should_delete_child, should_increase_depth) =
                            split(right, index - rope.weight, cut_nodes);

                        if should_increase_depth {
                            rope.depth = max(rope.depth, right.depth);
                        }
                        if should_delete_child {
                            let right = rope.right.take();
                            if let Some(cut_node) = right {
                                cut_nodes.push(cut_node);
                            }
                        }
                        rope.length = {
                            let left_len = match rope.left {
                                Some(ref left_child) => left_child.length,
                                None => 0,
                            };
                            let right_len = match rope.right {
                                Some(ref right_child) => right_child.length,
                                None => 0,
                            };
                            right_len + left_len
                        };

                        (rope.length == 0, should_increase_depth)
                    }
                    None => (false, false),
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

pub fn concatenate(left: Box<Node>, right: Box<Node>) -> Box<Node> {
    if let Some(ref left_str_content) = left.str_content
        && let Some(ref right_str_content) = right.str_content
    {
        let left_count = left.length;
        let right_count = right.length;

        if left_count + right_count <= LEAF_LEN {
            let combined_string: String =
                format!("{}{}", left_str_content.deref(), right_str_content.deref());
            return Box::new(Node::new(combined_string, left_count + right_count));
        }
    }
    if let Some(ref left_right_child) = left.right
        && let Some(ref str_content) = left_right_child.str_content
        && let Some(ref right_str_content) = right.str_content
    {
        let left_count = left_right_child.length;
        let right_count = right.length;

        if left_count + right_count <= LEAF_LEN {
            let combined_string: String =
                format!("{}{}", str_content.deref(), right_str_content.deref());
            let new_left_right_child = Node::new(combined_string, left_count + right_count);
            let new_node = Box::new(Node {
                depth: left.depth,
                length: left.length + right_count,
                left: left.left,
                weight: left.weight,
                right: Some(Box::new(new_left_right_child)),
                ..Default::default()
            });
            return rebalance(new_node);
        }
    }
    let new_concat = Node {
        depth: 1 + max(left.depth, right.depth),
        length: left.length + right.length,
        weight: left.length,
        left: Some(left),
        right: Some(right),
        ..Default::default()
    };
    rebalance(Box::new(new_concat))
}

pub fn insert(rope: Box<Node>, index: usize, content: String) -> Box<Node> {
    if rope.length == 0 {
        let content = content.graphemes(true).collect::<Vec<&str>>();
        return build_rope(&content, 0, content.len() - 1).0;
    }
    if index == 0 {
        let content = content.graphemes(true).collect::<Vec<&str>>();
        let new_node = build_rope(&content, 0, content.len() - 1).0;
        let new_rope = concatenate(new_node, rope);
        return new_rope;
    } else if index == rope.length {
        let content = content.graphemes(true).collect::<Vec<&str>>();
        let new_node = build_rope(&content, 0, content.len() - 1).0;
        let new_rope = concatenate(rope, new_node);
        return new_rope;
    }
    let mut original_rope = rope;
    let mut cut_nodes = Vec::new();

    let _ = split(&mut original_rope, index, &mut cut_nodes);

    let original_rope = rebalance(original_rope);

    let new_merged_cut_nodes = {
        let content = content.graphemes(true).collect::<Vec<&str>>();
        let (mut merged, _) = build_rope(&content, 0, content.len() - 1);
        for cut_node in cut_nodes {
            merged = concatenate(merged, cut_node);
        }
        merged
    };
    let new_merged_cut_nodes = rebalance(new_merged_cut_nodes);

    concatenate(original_rope, new_merged_cut_nodes)
}

pub fn remove(rope: Box<Node>, index: usize, length_to_cut: usize) -> Box<Node> {
    if rope.length == length_to_cut {
        return Box::new(Node::default());
    }
    if rope.length == 0 {
        return rope;
    }

    let mut original_rope = rope;

    if let Some(ref mut val) = original_rope.str_content {
        // Get the full string as graphemes
        let full_content: Vec<&str> = val.graphemes(true).collect();

        // Check bounds
        if index >= full_content.len() {
            return original_rope; // Nothing to remove
        }

        let end_index = std::cmp::min(index + length_to_cut, full_content.len());

        // Split into three parts
        let left_content = &full_content[0..index];
        let right_content = &full_content[end_index..];

        // Combine left and right
        let new_str: String = left_content
            .iter()
            .copied()
            .chain(right_content.iter().copied())
            .collect();

        // Create new leaf node
        let new_node = Node::new(new_str, left_content.len() + right_content.len());
        return Box::new(new_node);
    }
    let mut cut_nodes = Vec::new();

    let _ = split(&mut original_rope, index, &mut cut_nodes);
    let original_rope = rebalance(original_rope);

    if cut_nodes.is_empty() {
        return original_rope;
    }

    let mut new_merged_cut_nodes = {
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
            merged = concatenate(merged, cut_node);
        }
        merged
    };
    new_merged_cut_nodes = rebalance(new_merged_cut_nodes);

    let mut cut_nodes = Vec::new();
    let _ = split(&mut new_merged_cut_nodes, length_to_cut, &mut cut_nodes);

    if cut_nodes.is_empty() {
        return original_rope;
    }

    let mut third_new_merged_cut_nodes = {
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
            merged = concatenate(merged, cut_node);
        }
        merged
    };
    third_new_merged_cut_nodes = rebalance(third_new_merged_cut_nodes);
    if original_rope.length == 0 {
        return third_new_merged_cut_nodes;
    }

    concatenate(original_rope, third_new_merged_cut_nodes)
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
    if node.is_balanced() {
        return node;
    }
    if node.length == 0 {
        return node;
    }
    let mut slots: Vec<Option<Box<Node>>> = vec![None; 30];
    let mut leaves = Vec::new();
    collect_leaves(node, &mut leaves);
    'outer: for leaf in leaves {
        let slot_index = match FIBONACCI.binary_search(&leaf.length) {
            Ok(index) => index,
            Err(0) => 0,
            Err(i) => i - 1,
        };
        let mut nodes_to_concatenate = Vec::new();

        for slot in slots.iter_mut().take(slot_index) {
            if slot.is_some() {
                nodes_to_concatenate.push(slot.take().unwrap());
            }
        }
        if nodes_to_concatenate.is_empty() {
            let mut merged = leaf;
            for i in slot_index..slots.len() {
                let current = slots[i].take();
                match current {
                    Some(current_node) => {
                        merged = concatenate(current_node, merged);
                        let new_slot_index = match FIBONACCI.binary_search(&merged.length) {
                            Ok(index) => index,
                            Err(0) => 0,
                            Err(i) => i - 1,
                        };
                        if new_slot_index == i {
                            slots[i] = Some(merged);
                            continue 'outer;
                        }
                    }
                    None => {
                        let new_slot_index = match FIBONACCI.binary_search(&merged.length) {
                            Ok(index) => index,
                            Err(0) => 0,
                            Err(i) => i - 1,
                        };
                        if new_slot_index == i {
                            slots[i] = Some(merged);
                            continue 'outer;
                        }
                    }
                }
            }
        } else {
            let mut nodes_to_concatenate = nodes_to_concatenate.into_iter();
            let mut merged = nodes_to_concatenate.next().unwrap();
            for node in nodes_to_concatenate {
                merged = concatenate(node, merged);
            }
            merged = concatenate(merged, leaf);
            for i in slot_index..slots.len() {
                let current = slots[i].take();
                match current {
                    Some(current_node) => {
                        merged = concatenate(current_node, merged);
                        let new_slot_index = match FIBONACCI.binary_search(&merged.length) {
                            Ok(index) => index,
                            Err(0) => 0,
                            Err(i) => i - 1,
                        };
                        if new_slot_index == i {
                            slots[i] = Some(merged);
                            continue 'outer;
                        }
                    }
                    None => {
                        let new_slot_index = match FIBONACCI.binary_search(&merged.length) {
                            Ok(index) => index,
                            Err(0) => 0,
                            Err(i) => i - 1,
                        };
                        if new_slot_index == i {
                            slots[i] = Some(merged);
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }

    let mut result: Option<Box<Node>> = None;
    for slot in slots.into_iter().flatten() {
        result = Some(match result {
            None => slot,
            Some(r) => concatenate(slot, r),
        });
    }
    result.unwrap()
}

pub fn sub_rope(node: &Node, starting: usize, ending: usize) -> Node {
    match node.str_content {
        Some(ref content) => {
            let content = content.substr(starting..ending + 1);
            let length = (ending - starting) + 1;

            Node {
                weight: length,
                length,
                str_content: Some(content),
                ..Node::default()
            }
        }
        None => {
            if ending < node.weight {
                let left = node.left.as_ref().unwrap();
                sub_rope(left, starting, ending)
            } else if starting >= node.weight {
                let right = node.right.as_ref().unwrap();
                sub_rope(right, starting - node.weight, ending - node.weight)
            } else {
                let left = node.left.as_ref().unwrap();
                let left = sub_rope(left, starting, node.weight - 1);
                let right = node.right.as_ref().unwrap();
                let right = sub_rope(right, 0, ending - node.weight);

                Node {
                    length: left.length + right.length,
                    weight: left.length,
                    depth: 1 + max(left.depth, right.depth),
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                    ..Node::default()
                }
            }
        }
    }
}

pub fn find_sub_rope(node: &Node, starting: usize, ending: usize) -> Option<Node> {
    if starting >= node.length {
        return None;
    }
    let ending = min(node.length - 1, ending);
    Some(sub_rope(node, starting, ending))
}

pub fn find_sub_str(
    node: &Node,
    starting: usize,
    ending: usize,
    collected_string: &mut String,
) -> Option<()> {
    if starting >= node.length {
        return None;
    }
    let ending = min(node.length - 1, ending);
    sub_str(node, starting, ending, collected_string);
    Some(())
}

pub fn sub_str(node: &Node, starting: usize, ending: usize, collected_string: &mut String) {
    match node.str_content {
        Some(ref content) => {
            let content = content.get_part_of_string(starting..ending + 1);
            collected_string.push_str(content);
        }
        None => {
            if ending < node.weight {
                let left = node.left.as_ref().unwrap();
                sub_str(left, starting, ending, collected_string);
            } else if starting >= node.weight {
                let right = node.right.as_ref().unwrap();
                sub_str(
                    right,
                    starting - node.weight,
                    ending - node.weight,
                    collected_string,
                );
            } else {
                let left = node.left.as_ref().unwrap();
                sub_str(left, starting, node.weight - 1, collected_string);
                let right = node.right.as_ref().unwrap();
                sub_str(right, 0, ending - node.weight, collected_string);
            }
        }
    }
}
