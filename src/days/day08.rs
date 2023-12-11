use anyhow::Result;
use thiserror::Error;
use std::collections::HashMap;

struct Node<'n> {
    name: String,
    left: &'n Node<'n>,
    right: &'n Node<'n>,
}

type Instructions = String;

#[derive(Debug, Error)]
enum EighthError {
    #[error("missing instructions")]
    Instruction,

    #[error("missing nodes")]
    Node,

    #[error("missing name")]
    Name,

    #[error("connections")]
    Connection,
}

fn link_nodes<'n>(name: String, map: &HashMap<String, (String, String)>, node_list: &HashMap<String, Node>) -> Node<'n> {
    let connections = &map[&name];

    let left;
    if node_list.contains_key(&connections.0) {

    }

    Node {
        name,
        left: (),
        right: (),
    }
}

fn build_network<'n>(input: Vec<String>) -> Result<Node<'n>> {
    let mut node_set: HashMap<String, (String, String)> = HashMap::new();
    for node in input {
        let mut split = node.split(" = ");
        let name = split.next().ok_or(EighthError::Name)?.to_string();
        let children = split
            .next()
            .ok_or(EighthError::Connection)?
            .split(", ")
            .map(|c| c.trim_matches(|p| p == '(' || p == ')'))
            .collect::<Vec<&str>>();

        node_set.insert(name, (children[0].to_string(), children[1].to_string()));
    }

    todo!()
}

fn parse(input: &str) -> Result<(Instructions, Node)> {
    let mut stage_1 = input.split("\n\n");
    let instruction = stage_1.next().ok_or(EighthError::Instruction)?.to_string();
    let nodes = stage_1.next().ok_or(EighthError::Node)?.split('\n').map(String::from).collect();

    let network = build_network(nodes);

    Ok((instruction, network?))
}

pub fn first(input: &str) {
    let network = parse(input);
}
