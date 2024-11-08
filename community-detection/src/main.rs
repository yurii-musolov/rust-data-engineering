use community_detection::TWITTER_USERNAMES;
use petgraph::algo::kosaraju_scc;
use petgraph::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut graph = DiGraph::<&str, &str>::new();
    let mut nodes = HashMap::new();

    for window in TWITTER_USERNAMES.windows(2) {
        let user = window[0];
        let mention = window[1];

        let user_node = *nodes.entry(user).or_insert_with(|| graph.add_node(user));
        let mention_node = *nodes
            .entry(mention)
            .or_insert_with(|| graph.add_node(mention));

        graph.add_edge(user_node, mention_node, "retweets");
    }

    let scc = kosaraju_scc(&graph);
    let max = scc.iter().max();
    if let Some(community) = max {
        println!("{} nodes in community discovered", community.len());
        let usernames: Vec<&str> = community
            .iter()
            .map(|&node_index| graph[node_index])
            .collect();
        println!("{:?}", usernames);
    } else {
        println!("Community not found.");
    }
}
