use std::collections::{HashMap, VecDeque};

type Valve = (String, usize, Vec<String>);

pub fn reconstruct_path(came_from: VecDeque<Valve>, current: String) -> VecDeque<String> {
    let mut total_path = VecDeque::new();
    total_path.push_front(current);
    while let Some(cur)
    total_path
}
