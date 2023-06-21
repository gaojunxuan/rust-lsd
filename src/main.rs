use std::{env, fs::File, io::{BufReader, BufRead}, collections::HashMap};
use direct_superbubble::{data_structs::{Graph, Node}, detector::detect};

fn main() {
    // Read the file path from command-line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a path to the edge list as a command-line argument.");
        return;
    }
    let edge_list_path = &args[1];

    // Create an empty graph
    let mut graph = Graph { nodes: Vec::new() };

    // Read and parse the edge list file
    if let Ok(file) = File::open(edge_list_path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(edge) = line {
                let vertices: Vec<&str> = edge.split_whitespace().collect();
                if vertices.len() >= 2 {
                    let source_id: usize = vertices[0].parse().unwrap();
                    let target_id: usize = vertices[1].parse().unwrap();

                    // Add the nodes and update their connections
                    while graph.nodes.len() <= source_id.max(target_id) {
                        graph.nodes.push(Node {
                            id: graph.nodes.len(),
                            children: Vec::new(),
                            parents: Vec::new(),
                            properties: HashMap::new(),
                        });
                    }
                    graph.nodes[source_id].children.push(target_id);
                    graph.nodes[target_id].parents.push(source_id);
                }
            }
        }
    } else {
        eprintln!("Failed to open the edge list file.");
        return;
    }
    println!("Graph has {:?} nodes.", graph.nodes.len());
    detect(&mut graph);
}