
use std::io;

use ptree::{Color, PrintConfig, Style, TreeBuilder, print_config::UTF_CHARS_BOLD, print_tree, print_tree_with};
use text_editor::{app::App, rope::{Node, build_rope, collect_string, concatenate, find_length, index, insert, make_unbalanced_rope, rebalance, remove}};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::new(String::new()).run(terminal))
}
