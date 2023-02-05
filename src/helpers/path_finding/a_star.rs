use std::collections::HashMap;

type Valve = (String, usize, Vec<String>);

pub fn reconstruct_path(came_from: HashMap<String, Valve>, current: String)
