use rand::Rng;
use tokio::time::{sleep, Duration};

/// Simple vote type: either accept a value or reject.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Vote {
    Accept(String),
    Reject,
}

/// Each node can be honest or byzantine (faulty).
#[derive(Debug)]
struct Node {
    id: usize,
    byzantine: bool,
}

impl Node {
    fn new(id: usize, byzantine: bool) -> Self {
        Node { id, byzantine }
    }

    /// Simulate a node's decision on a proposed value.
    fn cast_vote(&self, proposal: &str) -> Vote {
        if self.byzantine {
            // Byzantine nodes misbehave randomly.
            if rand::thread_rng().gen_bool(0.5) {
                let fake = format!("fake-{}", rand::thread_rng().gen_range(0..50));
                Vote::Accept(fake)
            } else {
                Vote::Reject
            }
        } else {
            Vote::Accept(proposal.to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    // Parameters: n nodes, f faulty
    let n = 7;
    let f = 2; // needs n >= 3f+1 for safety
    let threshold = 2 * f + 1;

    println!("Simulating {} nodes with {} Byzantine", n, f);

    // First f nodes are byzantine, rest honest
    let mut nodes: Vec<Node> = Vec::new();
    for i in 0..n {
        nodes.push(Node::new(i, i < f));
    }

    // Round 1: leader proposes a value
    let proposal = "BLOCK-42".to_string();
    println!("\nLeader proposes: {}\n", proposal);

    // Collect votes
    let mut votes: Vec<Vote> = Vec::new();
    for node in &nodes {
        let v = node.cast_vote(&proposal);
        println!("Node {} (byz={}) -> {:?}", node.id, node.byzantine, v);
        votes.push(v);
    }

    // Count honest accepts for the proposal
    let mut accept_count = 0;
    for (i, v) in votes.iter().enumerate() {
        if let Vote::Accept(val) = v {
            if val == &proposal && !nodes[i].byzantine {
                accept_count += 1;
            }
        }
    }

    println!(
        "\nHonest accepts for {}: {}/{} (need >= {})",
        proposal, accept_count, n, threshold
    );

    if accept_count >= threshold {
        println!("✅ Consensus reached on {}\n", proposal);
    } else {
        println!("❌ Consensus failed this round\n");
    }

    // Run a few extra rounds to show leader rotation
    for round in 1..=3 {
        let new_proposal = format!("BLOCK-{}", 43 + round);
        println!("Round {} proposal: {}", round, new_proposal);

        let mut round_accepts = 0;
        for (i, node) in nodes.iter().enumerate() {
            let v = node.cast_vote(&new_proposal);
            if let Vote::Accept(val) = v {
                if val == &new_proposal && !node.byzantine {
                    round_accepts += 1;
                }
            }
        }

        if round_accepts >= threshold {
            println!("✅ Round {} accepted {}\n", round, new_proposal);
        } else {
            println!("⚠️ Round {} failed to reach consensus\n", round);
        }

        sleep(Duration::from_millis(400)).await;
    }

    println!("Simulation finished.");
}
