use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl std::str::FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Invalid position format".to_string());
        }
        let x = parts[0].parse::<i32>().map_err(|e| e.to_string())?;
        let y = parts[1].parse::<i32>().map_err(|e| e.to_string())?;
        Ok(Position { x, y })
    }
}

pub fn serialize_position_map<S>(
    map: &HashMap<Position, Tile>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeMap;
    let mut map_ser = serializer.serialize_map(Some(map.len()))?;
    for (pos, tile) in map {
        map_ser.serialize_entry(&pos.to_string(), tile)?;
    }
    map_ser.end()
}

pub fn deserialize_position_map<'de, D>(
    deserializer: D,
) -> Result<HashMap<Position, Tile>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{MapAccess, Visitor};
    use std::str::FromStr;

    struct PositionMapVisitor;

    impl<'de> Visitor<'de> for PositionMapVisitor {
        type Value = HashMap<Position, Tile>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map with string keys representing Position")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));
            while let Some((key_str, value)) = access.next_entry::<String, Tile>()? {
                let pos = Position::from_str(&key_str).map_err(serde::de::Error::custom)?;
                map.insert(pos, value);
            }
            Ok(map)
        }
    }

    deserializer.deserialize_map(PositionMapVisitor)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub pos: Position,
    pub hp: u32,
    pub max_hp: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileType {
    Forest,
    Ruins,
    Cave,
    Plains,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub tile_type: TileType,
    pub discovered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    #[serde(serialize_with = "serialize_position_map", deserialize_with = "deserialize_position_map")]
    pub tiles: HashMap<Position, Tile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldTemplate {
    #[serde(serialize_with = "serialize_position_map", deserialize_with = "deserialize_position_map")]
    pub tiles: HashMap<Position, Tile>,
}

impl WorldTemplate {
    pub fn from_world(world: &World) -> Self {
        Self {
            tiles: world.tiles.clone(),
        }
    }

    pub fn to_world(&self) -> World {
        World {
            tiles: self.tiles.clone(),
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameAction {
    Move(Direction),
    Explore,
    Invalid(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl GameState {
    pub fn new(player_name: String) -> Self {
        let mut world = World {
            tiles: HashMap::new(),
        };
        // Základní "mapa" pro testování
        world.tiles.insert(Position { x: 0, y: 0 }, Tile { tile_type: TileType::Plains, discovered: true });
        world.tiles.insert(Position { x: 0, y: 1 }, Tile { tile_type: TileType::Forest, discovered: false });
        world.tiles.insert(Position { x: 1, y: 0 }, Tile { tile_type: TileType::Ruins, discovered: false });

        Self::new_with_world(player_name, world)
    }

    pub fn new_with_world(player_name: String, world: World) -> Self {
        Self {
            player: Player {
                name: player_name.clone(),
                pos: Position { x: 0, y: 0 },
                hp: 100,
                max_hp: 100,
            },
            world,
            turn: 0,
            log: vec![format!("Welcome to the world of Dotiam, {}!", player_name)],
        }
    }

    pub fn get_current_description(&self) -> String {
        match self.world.tiles.get(&self.player.pos) {
            Some(tile) => match tile.tile_type {
                TileType::Plains => "You are standing on a vast plain. The wind plays with the blades of grass.".to_string(),
                TileType::Forest => "You are surrounded by a dense, dark forest. You hear branches snapping.".to_string(),
                TileType::Ruins => "You are in the midst of ancient ruins. The stones tell forgotten stories.".to_string(),
                TileType::Cave => "You are in a cold cave. Water drips from the ceiling.".to_string(),
            },
            None => "You are in an unknown wasteland.".to_string(),
        }
    }

    pub fn apply_action(&mut self, action: GameAction) {
        match action {
            GameAction::Move(dir) => {
                match dir {
                    Direction::North => {
                        self.player.pos.y += 1;
                        self.log.push("You go north.".to_string());
                    }
                    Direction::East => {
                        self.player.pos.x += 1;
                        self.log.push("You go east.".to_string());
                    }
                    Direction::South => {
                        self.player.pos.y -= 1;
                        self.log.push("You go south.".to_string());
                    }
                    Direction::West => {
                        self.player.pos.x -= 1;
                        self.log.push("You go west.".to_string());
                    }
                }
                self.turn += 1;
            }
            GameAction::Explore => {
                // TODO: Implement exploration logic
                self.log.push("Exploring the surroundings revealed nothing yet.".to_string());
                self.turn += 1;
            }
            GameAction::Invalid(cmd) => {
                self.log.push(format!("Unknown command: {}", cmd));
            }
        }
    }

    pub fn parse_command(&self, input: &str) -> GameAction {
        let input = input.trim().to_lowercase();
        match input.as_str() {
            "n" | "north" => GameAction::Move(Direction::North),
            "s" | "south" => GameAction::Move(Direction::South),
            "e" | "east" => GameAction::Move(Direction::East),
            "w" | "west" => GameAction::Move(Direction::West),
            "explore" | "look" | "x" => GameAction::Explore,
            "inventory" | "i" => {
                // TODO: Implement inventory display
                GameAction::Invalid("inventory (not implemented yet)".to_string())
            }
            _ => GameAction::Invalid(input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_player() {
        let mut state = GameState::new("Player1".to_string());
        state.apply_action(GameAction::Move(Direction::North));
        assert_eq!(state.player.pos, Position { x: 0, y: 1 });
        assert_eq!(state.turn, 1);
    }

    #[test]
    fn test_gamestate_serialization() {
        let state = GameState::new("TestPlayer".to_string());
        let json = serde_json::to_string(&state).expect("Failed to serialize");
        let deserialized: GameState = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(state.player.name, deserialized.player.name);
        assert_eq!(state.world.tiles.len(), deserialized.world.tiles.len());
        assert_eq!(state.player.pos, deserialized.player.pos);
        
        // Check if one of the tiles is correctly preserved
        let pos = Position { x: 0, y: 0 };
        assert!(deserialized.world.tiles.contains_key(&pos));
    }

    #[test]
    fn test_world_template_yaml() {
        let mut world = World { tiles: HashMap::new() };
        world.tiles.insert(Position { x: 5, y: 10 }, Tile { tile_type: TileType::Cave, discovered: true });
        
        let template = WorldTemplate::from_world(&world);
        let yaml = template.to_yaml();
        
        let deserialized = WorldTemplate::from_yaml(&yaml).expect("Failed to deserialize YAML");
        assert_eq!(deserialized.tiles.len(), 1);
        let pos = Position { x: 5, y: 10 };
        assert!(deserialized.tiles.contains_key(&pos));
        assert!(matches!(deserialized.tiles.get(&pos).unwrap().tile_type, TileType::Cave));
    }
}
