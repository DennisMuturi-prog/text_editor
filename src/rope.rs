use std::rc::Rc;
#[derive(Debug)]
pub struct Node{
    weight:usize,
    str_content:Option<Rc<str>>,
    left:Option<Box<Node>>,
    right:Option<Box<Node>>,
}

impl Node{
    fn new()->Self{
        Self { weight: 0, str_content: None, left: None, right: None}
        
    }
}
const LEAF_LEN:usize=3;

pub fn build_rope(content:&[char],starting:usize,ending:usize)->Box<Node>{
    
    let mut rope=Node::new();
    if ending-starting>=LEAF_LEN{
        let mid_point=(starting+ending)/2;
        let left=build_rope( content, starting, mid_point);
        let right=build_rope( content, mid_point+1, ending);
        rope.weight=(mid_point-starting)+1;
        rope.left=Some(left);
        rope.right=Some(right);  
    }else{
        let str_content:String=content[starting..=ending].iter().collect();
        let rc_from_string: Rc<str> = str_content.into();
        rope.weight=(ending-starting)+1;
        rope.str_content=Some(rc_from_string);
    }
    Box::new(rope)  
}
pub fn collect_string(node:&Node,content:&mut String){
    match node.str_content{
        Some(ref current) => {
            content.push_str(&current);
        },
        None => {
            if let Some(ref left_node) = node.left{
                collect_string(&left_node, content);
            }
            
            if let Some(ref right_node) = node.right{
                collect_string(&right_node, content);
            }
            
        },
    }
    
}

pub fn index(node:&Node,index:usize)->Option<char>{
    let mut index=index;
    let mut current_node=Some(node);
    while let Some(rope_node)=current_node{
        match rope_node.str_content{
            Some(ref content) => {
                let mut characters=content.chars();
                let letter=characters.nth(index);
                return letter;
            },
            None => {
                if index<rope_node.weight{
                    current_node=rope_node.left.as_deref();
                }else{
                    index-=rope_node.weight;
                    let right=rope_node.right.as_deref();
                    current_node=right;
                }
            },
        }
        
    }
    None
    
    
    
}

pub fn split(rope:&mut Node,index:usize,cut_nodes:&mut Vec<Box<Node>>)->(usize,bool){
    match rope.str_content{
        Some(ref content) => {
            if index==0{
                // let weight_to_reduce=rope.weight;
                // if branch_to_snip==0{
                //     let cut_node=parent.left.take();
                // }else{
                    
                // }
                return (rope.weight,true); 
            }else{
                return (rope.weight,true); 
                
            }
            
        },
        None => {
            if index<rope.weight{
                match rope.left{
                    Some(ref mut left) => {
                        let (weight_to_reduce,should_delete_child)=split(left, index,cut_nodes);
                        rope.weight-=weight_to_reduce;
                        if should_delete_child{
                            let right=rope.left.take();
                            if let Some(cut_node)=right{
                                cut_nodes.push(cut_node);
                            }
                            
                        }
                        let right=rope.right.take();
                        if let Some(cut_node)=right{
                            cut_nodes.push(cut_node);
                        }
                        (weight_to_reduce,false)
                    },
                    None => {
                        (0,false)
                    },
                }
            }else{
                match rope.right{
                    Some(ref mut right) => {
                        let (weight_to_reduce,should_delete_child)= split(right, index,cut_nodes); 
                        if should_delete_child{
                            let right=rope.right.take();
                            if let Some(cut_node)=right{
                                cut_nodes.push(cut_node);
                            }
                            
                        }
                        
                        (weight_to_reduce,false)
                    },
                    None => {
                        (0,false)
                    },
                }
            }
        },
    }
    
}

fn find_length(node:&Node,count){
    
}

fn concatenate(left:Node,right:Node)->Node{
    
}