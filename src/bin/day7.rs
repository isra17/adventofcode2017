use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node {
    pub name: String,
    pub weight: u64,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn parse(text: &str) -> Option<Rc<RefCell<Node>>> {
        let mut nodes: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
        let raw_nodes: Vec<(String, u64, Vec<String>)> = text.lines()
            .filter_map(|l| {
                let mut words = l.split_whitespace();
                let name = words.next().unwrap();
                let weight = words.next()
                    .unwrap()
                    .trim_right_matches(')')
                    .trim_left_matches('(')
                    .parse::<u64>()
                    .unwrap();
                let mut children: Vec<String> = Vec::new();
                if words.next().is_some() {
                    children.extend::<Vec<String>>(words.map(|w| w.trim_right_matches(',').into())
                        .collect());
                }
                Some((name.into(), weight, children))
            })
            .collect();

        raw_nodes.iter().for_each(|&(ref name, weight, _)| {
            nodes.insert(name.clone(),
                         Rc::new(RefCell::new(Node {
                             name: name.clone(),
                             weight: weight,
                             children: Vec::new(),
                         })));
        });

        let mut roots: HashSet<String> = nodes.keys().cloned().collect();
        raw_nodes.into_iter().for_each(|(name, _, children)| {
            children.iter().for_each(|child_name| {
                roots.remove(child_name);

                let child = nodes.get(child_name).unwrap().clone();
                let ref mut n = nodes.get(&name).unwrap().borrow_mut();
                n.children.push(child);
            });
        });


        Some(nodes.get(roots.iter().next().unwrap()).unwrap().clone())
    }
}

fn check_balance(tree: &Rc<RefCell<Node>>) -> Result<u64, Rc<RefCell<Node>>> {
    let node = tree.borrow();
    if node.children.len() == 0 {
        return Ok(node.weight);
    }

    let mut child_weight = None;
    for child in node.children.iter() {
        let weight = check_balance(&child)?;
        if child_weight.is_none() {
            child_weight = Some(weight);
        } else if weight != child_weight.unwrap() {
            return Err(tree.clone());
        }
    }

    Ok(node.children.iter().map(|n| check_balance(n).expect("")).sum::<u64>() + node.weight)
}

fn fix_weight(tree: &Rc<RefCell<Node>>) -> u64 {
    let node = tree.borrow();
    let mut weights: Vec<(u64, u64)> = node.children
        .iter()
        .map(|ref n| (check_balance(n).expect(""), n.borrow().weight))
        .collect();
    weights.sort_by_key(|&(w, _)| w);
    let (bad, good) = if weights[0].0 != weights[1].0 {
        (weights[0], weights[weights.len() - 1])
    } else {
        (weights[weights.len() - 1], weights[0])
    };

    bad.1 + good.0 - bad.0
}


fn main() {
    let mut file = File::open("day7.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let root = Node::parse(&contents).unwrap();
    println!("Part 1: {}", root.borrow().name);

    let node = check_balance(&root);
    let weight = fix_weight(&node.unwrap_err());
    println!("Part 2: {:?}", weight);
}

#[cfg(test)]
mod tests {
    use Node;
    #[test]
    fn test_parse_input() {
        let root = Node::parse("pbga (66)
                                xhth (57)
                                ebii (61)
                                havc (66)
                                ktlj (57)
                                fwft (72) -> ktlj, cntj, xhth
                                qoyq (66)
                                padx (45) -> pbga, havc, qoyq
                                tknk (41) -> ugml, padx, fwft
                                jptl (61)
                                ugml (68) -> gyxo, ebii, jptl
                                gyxo (61)
                                cntj (57)")
            .unwrap();
        assert_eq!(root.borrow().name, "tknk");
    }

}
