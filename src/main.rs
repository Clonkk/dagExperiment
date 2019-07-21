mod data_struct;
use data_struct::Graph;

fn main() {
    println!("Hello, world!");
    let mut graph: Graph = Graph::new();
    let filename = "database4.txt".to_string();
    graph.parse(filename);
    graph.show();

    let avg_depth = graph.get_avg_depth();
    println!("avg_depth={}", avg_depth);

    let avg_tx = graph.get_avg_tx_per_depth();
    println!("avg_tx={}", avg_tx);

    let avg_ref = graph.get_avg_reference_per_node();
    println!("avg_ref={}", avg_ref);

    let num_leaf = graph.get_num_of_leaf();
    println!("num_leaf={}", num_leaf);
}
