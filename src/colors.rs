use std::collections::HashMap;

pub struct Colors {
    colors: Vec<String>,
    current_index: usize,
    taken_colors: HashMap<String, usize>,
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            colors: vec![
                String::from("#b94238"),
                String::from("#168952"),
                String::from("#3e76cf"),
                String::from("#9244a2"),
                String::from("#cd9c0c"),
                String::from("#ff855f"),
                String::from("#FF1493"),
                String::from("#00BFFF"),
                String::from("#00FF00"),
                String::from("#FFD700"),
            ],
            current_index: 0,
            taken_colors: HashMap::new(),
        }
    }
}

impl Colors {
    pub fn get_color(&mut self, entry_name: String) -> &str {
        let used_before = self.taken_colors.get(&entry_name);

        match used_before {
            Some(index) => &self.colors[*index],
            None => {
                self.taken_colors.insert(entry_name, self.current_index);

                let color = &self.colors[self.current_index];

                self.current_index += 1;
                if self.current_index > self.colors.len() - 1 {
                    self.current_index = 0;
                }

                color
            }
        }
    }
}
