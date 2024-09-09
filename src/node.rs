use crate::colors::Colors;

#[derive(Clone)]
pub struct Node {
    id: String,
    name: String,
    votes: u32,
    color: String,
    x_pos: f64,
    y_pos: f64,
    height: f64,
    label: String
}

impl Node {
    pub fn new(label: &str, color: &mut Colors, index: usize) -> Self {        
        let mut label_split = label.split(" (");
        let name= label_split.next().unwrap_or("?????");
        let votes = label_split.next().unwrap();
        let votes = &votes[..votes.len() - 1];
        let votes = votes.to_string().parse::<u32>().unwrap_or(0);
        
        Self {
            id: format!("{}-{}", name, index),
            name: name.to_string(),
            votes,
            color: color.get_color(name.to_string()).to_string(),
            x_pos: 0.0,
            y_pos: 0.0,
            height: 0.0,
            label: label.to_string()
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
    
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn votes(&self) -> u32 {
        self.votes
    }

    pub fn color(&self) -> String {
        self.color.clone()
    }
}