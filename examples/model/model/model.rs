pub struct World {
    pub active_scene: Option<usize>,
    pub scenes: Vec<Scene>,
}

pub struct Scene {
    pub name: Option<String>,
    pub nodes: Vec<Node>,
}

pub struct Node {
    pub name: Option<String>,
}