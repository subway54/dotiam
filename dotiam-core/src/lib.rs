use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct Condition {
    pub condition_type: ConditionType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConditionType {
    HasItem(String),
    HasAttribute(String, String),
    MinHP(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Edge {
    pub target_id: String,
    pub label: String,
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub can_pickup: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Combination {
    pub item1: String,
    pub item2: String,
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub description: String,
    pub attributes: HashMap<String, String>,
    pub edges: Vec<Edge>,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub current_node: String,
    pub hp: u32,
    pub max_hp: u32,
    pub inventory: Vec<String>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub nodes: HashMap<String, Node>,
    pub items: HashMap<String, Item>,
    pub combinations: Vec<Combination>,
}

impl World {
    pub fn get_ascii_map(&self) -> String {
        "Graph-based world (ASCII map disabled)".to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldTemplate {
    pub nodes: HashMap<String, Node>,
    #[serde(default)]
    pub items: HashMap<String, Item>,
    #[serde(default)]
    pub combinations: Vec<Combination>,
}

impl WorldTemplate {
    pub fn from_world(world: &World) -> Self {
        Self {
            nodes: world.nodes.clone(),
            items: world.items.clone(),
            combinations: world.combinations.clone(),
        }
    }

    pub fn to_world(&self) -> World {
        World {
            nodes: self.nodes.clone(),
            items: self.items.clone(),
            combinations: self.combinations.clone(),
        }
    }

    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).expect("Failed to serialize WorldTemplate to YAML")
    }

    pub fn from_yaml(content: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(content)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub player: Player,
    pub world: World,
    pub turn: u32,
    pub log: Vec<String>,
    pub history: Vec<World>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameAction {
    Help,
    Look,
    Move(String), // target_id
    Explore(Option<String>),
    Combine(String, String),
    Pickup(String),
    Drop(String),
    Inventory,
    Use(String),
    Invalid(String),
}

impl GameState {
    pub fn new(player_name: String) -> Self {
        let mut nodes = HashMap::new();
        let start_node = Node {
            id: "start".to_string(),
            description: "You are at the starting point of your adventure.".to_string(),
            attributes: HashMap::new(),
            edges: vec![Edge {
                target_id: "forest".to_string(),
                label: "Go to the forest".to_string(),
                conditions: vec![],
            }],
            items: vec![],
        };
        let forest_node = Node {
            id: "forest".to_string(),
            description: "You are in a dark, mysterious forest.".to_string(),
            attributes: HashMap::new(),
            edges: vec![Edge {
                target_id: "start".to_string(),
                label: "Return to the start".to_string(),
                conditions: vec![],
            }],
            items: vec![],
        };
        nodes.insert(start_node.id.clone(), start_node);
        nodes.insert(forest_node.id.clone(), forest_node);

        let world = World {
            nodes,
            items: HashMap::new(),
            combinations: vec![],
        };

        Self {
            player: Player {
                name: player_name.clone(),
                current_node: "start".to_string(),
                hp: 100,
                max_hp: 100,
                inventory: Vec::new(),
                attributes: HashMap::new(),
            },
            world,
            turn: 0,
            log: vec![format!("Welcome to the world of Dotiam, {}!", player_name)],
            history: Vec::new(),
        }
    }

    pub fn new_with_world(player_name: String, world: World) -> Self {
        let current_node = if world.nodes.contains_key("start") {
            "start".to_string()
        } else {
            world.nodes.keys().next().cloned().unwrap_or_default()
        };

        Self {
            player: Player {
                name: player_name.clone(),
                current_node,
                hp: 100,
                max_hp: 100,
                inventory: Vec::new(),
                attributes: HashMap::new(),
            },
            world,
            turn: 0,
            log: vec![format!("Welcome to the world of Dotiam, {}!", player_name)],
            history: Vec::new(),
        }
    }

    pub fn get_current_description(&self) -> String {
        match self.world.nodes.get(&self.player.current_node) {
            Some(node) => node.description.clone(),
            None => "You are lost in the void.".to_string(),
        }
    }

    pub fn can_traverse(&self, edge: &Edge) -> bool {
        for condition in &edge.conditions {
            match &condition.condition_type {
                ConditionType::HasItem(item) => {
                    if !self.player.inventory.contains(item) {
                        return false;
                    }
                }
                ConditionType::HasAttribute(key, value) => {
                    if self.player.attributes.get(key) != Some(value) {
                        return false;
                    }
                }
                ConditionType::MinHP(min_hp) => {
                    if self.player.hp < *min_hp {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn apply_action(&mut self, action: GameAction) {
        match action {
            GameAction::Help => {
                self.log.push("Available commands:".to_string());
                self.log.push("  h, help          - Show this help".to_string());
                self.log.push("  l, look          - Look at the current scene".to_string());
                self.log.push("  g, go <target>   - Go to a specific place".to_string());
                self.log.push("  x, explore [obj] - Explore the scene or an object".to_string());
                self.log.push("  c, combine <a> <b> - Combine two items".to_string());
                self.log.push("  p, pickup <item> - Pick up an item".to_string());
                self.log.push("  d, drop <item>   - Drop an item".to_string());
                self.log.push("  i, inventory     - Show your inventory".to_string());
                self.log.push("  u, use <item>    - Use an item".to_string());
            }
            GameAction::Look => {
                self.log.push(self.get_current_description());
                if let Some(node) = self.world.nodes.get(&self.player.current_node) {
                    if !node.items.is_empty() {
                        let item_names: Vec<String> = node.items.iter()
                            .map(|id| self.world.items.get(id).map(|i| i.name.clone()).unwrap_or(id.clone()))
                            .collect();
                        self.log.push(format!("Items here: {}", item_names.join(", ")));
                    }
                    let paths: Vec<String> = node.edges.iter().map(|e| e.label.clone()).collect();
                    self.log.push(format!("Available paths: {}", paths.join(", ")));
                }
            }
            GameAction::Move(target_id) => {
                let current_node = self.world.nodes.get(&self.player.current_node).cloned();
                if let Some(node) = current_node {
                    if let Some(edge) = node.edges.iter().find(|e| e.target_id == target_id) {
                        if self.can_traverse(edge) {
                            self.player.current_node = target_id.clone();
                            self.log.push(format!("You move to: {}.", edge.label));
                            self.turn += 1;
                        } else {
                            self.log.push(format!("You cannot go to {}, conditions not met.", edge.label));
                        }
                    } else {
                        self.log.push("You cannot go that way.".to_string());
                    }
                }
            }
            GameAction::Explore(target) => {
                if let Some(target_id) = target {
                    // Explore specific item or feature
                    if let Some(item) = self.world.items.get(&target_id) {
                        if self.player.inventory.contains(&target_id) || 
                           self.world.nodes.get(&self.player.current_node).map_or(false, |n| n.items.contains(&target_id)) {
                            self.log.push(format!("{}: {}", item.name, item.description));
                        } else {
                            self.log.push(format!("You don't see any {} here.", target_id));
                        }
                    } else {
                        self.log.push(format!("You don't see anything special about {}.", target_id));
                    }
                } else {
                    self.log.push("You look around carefully but find nothing new.".to_string());
                }
                self.turn += 1;
            }
            GameAction::Pickup(item_id) => {
                let current_node_id = self.player.current_node.clone();
                if let Some(node) = self.world.nodes.get_mut(&current_node_id) {
                    if let Some(pos) = node.items.iter().position(|id| id == &item_id) {
                        let can_pickup = self.world.items.get(&item_id).map_or(true, |i| i.can_pickup);
                        if can_pickup {
                            node.items.remove(pos);
                            self.player.inventory.push(item_id.clone());
                            let name = self.world.items.get(&item_id).map_or(item_id.clone(), |i| i.name.clone());
                            self.log.push(format!("You picked up: {}", name));
                            self.turn += 1;
                        } else {
                            self.log.push("You cannot pick that up.".to_string());
                        }
                    } else {
                        self.log.push("That item is not here.".to_string());
                    }
                }
            }
            GameAction::Drop(item_id) => {
                if let Some(pos) = self.player.inventory.iter().position(|id| id == &item_id) {
                    self.player.inventory.remove(pos);
                    if let Some(node) = self.world.nodes.get_mut(&self.player.current_node) {
                        node.items.push(item_id.clone());
                    }
                    let name = self.world.items.get(&item_id).map_or(item_id.clone(), |i| i.name.clone());
                    self.log.push(format!("You dropped: {}", name));
                    self.turn += 1;
                } else {
                    self.log.push("You don't have that item.".to_string());
                }
            }
            GameAction::Inventory => {
                if self.player.inventory.is_empty() {
                    self.log.push("Your inventory is empty.".to_string());
                } else {
                    let item_names: Vec<String> = self.player.inventory.iter()
                        .map(|id| self.world.items.get(id).map(|i| i.name.clone()).unwrap_or(id.clone()))
                        .collect();
                    self.log.push(format!("You are carrying: {}", item_names.join(", ")));
                }
            }
            GameAction::Use(item_id) => {
                if self.player.inventory.contains(&item_id) {
                    self.log.push(format!("You use the {}. Nothing obvious happens.", item_id));
                    self.turn += 1;
                } else {
                    self.log.push("You don't have that item.".to_string());
                }
            }
            GameAction::Combine(item1, item2) => {
                if self.player.inventory.contains(&item1) && self.player.inventory.contains(&item2) {
                    let combination = self.world.combinations.iter().find(|c| 
                        (c.item1 == item1 && c.item2 == item2) || (c.item1 == item2 && c.item2 == item1)
                    ).cloned();

                    if let Some(combo) = combination {
                        self.player.inventory.retain(|id| id != &item1 && id != &item2);
                        self.player.inventory.push(combo.result.clone());
                        let result_name = self.world.items.get(&combo.result).map_or(combo.result.clone(), |i| i.name.clone());
                        self.log.push(format!("You combined them and created: {}!", result_name));
                        self.turn += 1;
                    } else {
                        self.log.push("Those items cannot be combined.".to_string());
                    }
                } else {
                    self.log.push("You need both items in your inventory to combine them.".to_string());
                }
            }
            GameAction::Invalid(cmd) => {
                self.log.push(format!("Unknown command: {}", cmd));
            }
        }
    }

    pub fn parse_command(&self, input: &str) -> GameAction {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return GameAction::Invalid("".to_string());
        }

        let cmd = parts[0].to_lowercase();
        let args = &parts[1..];

        match cmd.as_str() {
            "h" | "help" => GameAction::Help,
            "l" | "look" => GameAction::Look,
            "g" | "go" => {
                if args.is_empty() {
                    GameAction::Invalid("Go where?".to_string())
                } else {
                    let target = args.join(" ");
                    // Try to find a matching edge label or target_id
                    if let Some(node) = self.world.nodes.get(&self.player.current_node) {
                        for edge in &node.edges {
                            if edge.label.to_lowercase() == target.to_lowercase() || edge.target_id.to_lowercase() == target.to_lowercase() {
                                return GameAction::Move(edge.target_id.clone());
                            }
                        }
                    }
                    GameAction::Move(target)
                }
            }
            "x" | "explore" => {
                if args.is_empty() {
                    GameAction::Explore(None)
                } else {
                    GameAction::Explore(Some(args.join(" ")))
                }
            }
            "p" | "pickup" | "get" | "take" => {
                if args.is_empty() {
                    GameAction::Invalid("Pick up what?".to_string())
                } else {
                    GameAction::Pickup(args.join(" "))
                }
            }
            "d" | "drop" => {
                if args.is_empty() {
                    GameAction::Invalid("Drop what?".to_string())
                } else {
                    GameAction::Drop(args.join(" "))
                }
            }
            "i" | "inventory" | "inv" => GameAction::Inventory,
            "u" | "use" => {
                if args.is_empty() {
                    GameAction::Invalid("Use what?".to_string())
                } else {
                    GameAction::Use(args.join(" "))
                }
            }
            "c" | "combine" => {
                if args.len() < 2 {
                    GameAction::Invalid("Combine what with what?".to_string())
                } else {
                    GameAction::Combine(args[0].to_string(), args[1].to_string())
                }
            }
            _ => {
                // Fallback for direct movement if just the target/label is typed
                let target = parts.join(" ");
                if let Some(node) = self.world.nodes.get(&self.player.current_node) {
                    for edge in &node.edges {
                        if edge.label.to_lowercase() == target.to_lowercase() || edge.target_id.to_lowercase() == target.to_lowercase() {
                            return GameAction::Move(edge.target_id.clone());
                        }
                    }
                }
                GameAction::Invalid(target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_player() {
        let mut state = GameState::new("Player1".to_string());
        state.apply_action(GameAction::Move("forest".to_string()));
        assert_eq!(state.player.current_node, "forest");
        assert_eq!(state.turn, 1);
    }

    #[test]
    fn test_gamestate_serialization() {
        let state = GameState::new("TestPlayer".to_string());
        let json = serde_json::to_string(&state).expect("Failed to serialize");
        let deserialized: GameState = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(state.player.name, deserialized.player.name);
        assert_eq!(state.world.nodes.len(), deserialized.world.nodes.len());
        assert_eq!(state.player.current_node, deserialized.player.current_node);
    }

    #[test]
    fn test_world_template_yaml() {
        let mut nodes = HashMap::new();
        nodes.insert("cave".to_string(), Node {
            id: "cave".to_string(),
            description: "A dark cave".to_string(),
            attributes: HashMap::new(),
            edges: vec![],
            items: vec![],
        });
        let world = World {
            nodes,
            items: HashMap::new(),
            combinations: vec![],
        };
        
        let template = WorldTemplate::from_world(&world);
        let yaml = template.to_yaml();
        
        let deserialized = WorldTemplate::from_yaml(&yaml).expect("Failed to deserialize YAML");
        assert_eq!(deserialized.nodes.len(), 1);
        assert!(deserialized.nodes.contains_key("cave"));
    }

    #[test]
    fn test_complex_interactions() {
        let mut state = GameState::new("Tester".to_string());
        
        // Add items and combination to world
        state.world.items.insert("stick".to_string(), Item {
            id: "stick".to_string(),
            name: "Stick".to_string(),
            description: "A stick".to_string(),
            can_pickup: true,
        });
        state.world.items.insert("stone".to_string(), Item {
            id: "stone".to_string(),
            name: "Stone".to_string(),
            description: "A stone".to_string(),
            can_pickup: true,
        });
        state.world.items.insert("torch".to_string(), Item {
            id: "torch".to_string(),
            name: "Torch".to_string(),
            description: "A torch".to_string(),
            can_pickup: true,
        });
        state.world.combinations.push(Combination {
            item1: "stick".to_string(),
            item2: "stone".to_string(),
            result: "torch".to_string(),
        });
        
        // Put items in current node
        state.world.nodes.get_mut("start").unwrap().items.push("stick".to_string());
        state.world.nodes.get_mut("start").unwrap().items.push("stone".to_string());
        
        // Pickup items
        state.apply_action(GameAction::Pickup("stick".to_string()));
        state.apply_action(GameAction::Pickup("stone".to_string()));
        assert!(state.player.inventory.contains(&"stick".to_string()));
        assert!(state.player.inventory.contains(&"stone".to_string()));
        
        // Combine items
        state.apply_action(GameAction::Combine("stick".to_string(), "stone".to_string()));
        assert!(!state.player.inventory.contains(&"stick".to_string()));
        assert!(!state.player.inventory.contains(&"stone".to_string()));
        assert!(state.player.inventory.contains(&"torch".to_string()));
        
        // Use torch to unlock path (simulation of conditions)
        let edge = Edge {
            target_id: "cave".to_string(),
            label: "Cave".to_string(),
            conditions: vec![Condition {
                condition_type: ConditionType::HasItem("torch".to_string()),
            }],
        };
        assert!(state.can_traverse(&edge));
    }

    #[test]
    fn test_yaml_deserialization() {
        let yaml = r#"
nodes:
  start:
    id: start
    description: "Start"
    attributes: {}
    edges:
      - target_id: forest
        label: "Forest"
        conditions: []
    items: []
  forest:
    id: forest
    description: "Forest"
    attributes: {}
    edges:
      - target_id: cave
        label: "Cave"
        conditions:
          - !HasItem torch
    items: []
items:
  torch:
    id: torch
    name: "Torch"
    description: "Torch"
    can_pickup: true
combinations: []
"#;
        let template = WorldTemplate::from_yaml(yaml).unwrap();
        let forest = template.nodes.get("forest").unwrap();
        assert_eq!(forest.edges[0].conditions.len(), 1);
        match &forest.edges[0].conditions[0].condition_type {
            ConditionType::HasItem(item) => assert_eq!(item, "torch"),
            _ => panic!("Expected HasItem condition"),
        }
    }

    #[test]
    fn test_whispering_woods_walkthrough() {
        let content = std::fs::read_to_string("../world.yaml").expect("Failed to read world.yaml");
        let template = WorldTemplate::from_yaml(&content).expect("Failed to parse world.yaml");
        let mut state = GameState::new_with_world("Hero".to_string(), template.to_world());

        // 1. Start at the Forest Crossroads. Pick up the flint.
        assert_eq!(state.player.current_node, "start");
        state.apply_action(GameAction::Pickup("flint".to_string()));
        assert!(state.player.inventory.contains(&"flint".to_string()));

        // 2. Go to the Stone Bridge and pick up the dry_wood.
        state.apply_action(GameAction::Move("bridge".to_string()));
        assert_eq!(state.player.current_node, "bridge");
        state.apply_action(GameAction::Pickup("dry_wood".to_string()));
        assert!(state.player.inventory.contains(&"dry_wood".to_string()));

        // 3. Combine flint and dry_wood to create a torch.
        state.apply_action(GameAction::Combine("flint".to_string(), "dry_wood".to_string()));
        assert!(state.player.inventory.contains(&"torch".to_string()));
        assert!(!state.player.inventory.contains(&"flint".to_string()));
        assert!(!state.player.inventory.contains(&"dry_wood".to_string()));

        // 4. Go to the Forgotten Path, pick up wild_herbs, and use the torch to enter the Echoing Cave.
        state.apply_action(GameAction::Move("start".to_string()));
        state.apply_action(GameAction::Move("forgotten_path".to_string()));
        assert_eq!(state.player.current_node, "forgotten_path");
        state.apply_action(GameAction::Pickup("wild_herbs".to_string()));
        assert!(state.player.inventory.contains(&"wild_herbs".to_string()));

        state.apply_action(GameAction::Move("cave".to_string()));
        assert_eq!(state.player.current_node, "cave");

        // 5. Find the iron_key inside the cave.
        state.apply_action(GameAction::Pickup("iron_key".to_string()));
        assert!(state.player.inventory.contains(&"iron_key".to_string()));

        // 6. Go to the Hut Exterior and use the iron_key to enter the Hut Interior.
        state.apply_action(GameAction::Move("forgotten_path".to_string()));
        state.apply_action(GameAction::Move("start".to_string()));
        state.apply_action(GameAction::Move("hut_exterior".to_string()));
        assert_eq!(state.player.current_node, "hut_exterior");
        
        state.apply_action(GameAction::Move("hut_interior".to_string()));
        assert_eq!(state.player.current_node, "hut_interior");

        // 7. Pick up the cauldron and combine it with wild_herbs to brew the purifying_potion.
        state.apply_action(GameAction::Pickup("cauldron".to_string()));
        assert!(state.player.inventory.contains(&"cauldron".to_string()));
        
        state.apply_action(GameAction::Combine("wild_herbs".to_string(), "cauldron".to_string()));
        assert!(state.player.inventory.contains(&"purifying_potion".to_string()));

        // 8. Travel through the Stone Bridge to the Castle Gate.
        state.apply_action(GameAction::Move("hut_exterior".to_string()));
        state.apply_action(GameAction::Move("start".to_string()));
        state.apply_action(GameAction::Move("bridge".to_string()));
        state.apply_action(GameAction::Move("castle_gate".to_string()));
        assert_eq!(state.player.current_node, "castle_gate");

        // 9. Use the purifying_potion to break the seal and enter the Castle Keep.
        state.apply_action(GameAction::Move("castle_keep".to_string()));
        assert_eq!(state.player.current_node, "castle_keep");

        // 10. Retrieve the artifact and lift the curse.
        state.apply_action(GameAction::Pickup("artifact".to_string()));
        assert!(state.player.inventory.contains(&"artifact".to_string()));
    }
}
