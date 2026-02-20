# Dotiam - 06

## Zadání 6.1

(Mystická lesní adventura – návrh jednoho levelu)

You are a professional game designer specializing in narrative-driven text adventure games.

Your task is to design one complete, logically consistent level set in a mystical fantasy forest.
The output must be structured, internally consistent, and mechanically playable.

🌲 Setting

The level takes place in a deep, ancient forest infused with subtle magic.
The atmosphere is mysterious, immersive, slightly tense but not horror.

Deep within the forest there are three major points of interest:

A medieval castle

An old hut

A cave

These are the primary progression locations.

🗺 World Design Rules

The player cannot travel directly between major locations.
Movement must require passing through intermediary locations (clearings, crossroads, stone bridges, swamps, forgotten paths, ruins, etc.).

The world must feel interconnected and spatially believable.

Some paths must be blocked initially and require:

specific items

crafted/combined objects

solved environmental conditions

The design must avoid softlocks (no unwinnable states).

🧩 Item & Crafting System

Each major location must:

contain unique items

contain at least one meaningful interaction

potentially hide gated areas

Design:

At least 6–10 base items distributed across locations

At least 2–3 logical item combinations

Each combination must produce a meaningful new item

At least one crafted item must be required to access a previously blocked location

Combinations must make sense within a mystical medieval setting.
Avoid arbitrary crafting.

🎯 Objective

Define a clear overarching goal for the level, such as:

lifting a forest curse

retrieving an ancient artifact

awakening or banishing a forest spirit

unlocking a sealed magical gate

The goal must require:

visiting multiple major locations

crafting at least one combined item

unlocking at least one blocked path

📦 Output Structure (MANDATORY)

Structure your response exactly as follows:

1. Atmosphere Overview

Short immersive description (5–10 sentences).

2. Location List

List all locations (major + intermediary).
For each location provide:

Name

Short description

Connected locations

3. World Navigation Map (Textual Graph)

Provide adjacency-style mapping, e.g.:
Forest Crossroads → Old Bridge → Castle Gate

4. Items by Location

List items per location.

5. Item Combinations

Clearly specify:

Required items

Resulting item

Purpose

6. Locked Paths & Conditions

Describe:

What is blocked

What is required

How player unlocks it

7. Level Objective & Resolution Flow

Step-by-step logical progression from start to completion.

⚖ Design Constraints

Maintain internal logic consistency.

Avoid random puzzle logic.

Avoid excessive complexity.

No modern technology.

No breaking tone (no humor unless subtle and thematic).

Keep tone mystical, immersive, and atmospheric.

🎮 Design Philosophy

The level should reward:

exploration

observation

logical reasoning

spatial memory

The result must feel like a real playable adventure level blueprint.

## Zadání 6.2

Zde je návrh nové úrovně a odpovídající soubor `world.yaml`.

### 1. Atmosphere Overview
The air in the "Whispering Woods" is thick with the scent of pine and ancient magic. Sunlight struggles to pierce through the dense canopy, creating a perpetual twilight. Every rustle of leaves sounds like a half-forgotten secret, and the very earth feels as if it’s pulse is synchronized with a long-lost heartbeat. It is a place of deep peace and underlying tension, where the boundary between the natural and the mystical is paper-thin.

### 2. Location List
*   **Forest Crossroads (Start):** A central point where three paths meet. Contains a piece of flint.
*   **Stone Bridge:** An ancient, mossy bridge leading north towards the castle. Contains dry wood.
*   **Forgotten Path:** A treacherous, overgrown trail leading west to the cave. Contains wild herbs.
*   **Hut Exterior:** The area outside an old, locked wooden hut to the east.
*   **Hut Interior:** A dusty but preserved sanctuary filled with alchemical tools. Contains a cauldron and an empty bottle.
*   **Echoing Cave:** A dark, damp cavern that requires a light source to enter. Contains an iron key.
*   **Castle Gate:** A grand entrance sealed by a shimmering magical barrier.
*   **Castle Keep (Objective):** The heart of the castle where the forest's curse is finally broken.

### 3. World Navigation Map (Textual Graph)
*   `Echoing Cave` ← `Forgotten Path` ↔ **`Forest Crossroads`** ↔ `Hut Exterior` ↔ `Hut Interior`
*   **`Forest Crossroads`** ↔ `Stone Bridge` ↔ `Castle Gate` ↔ `Castle Keep`

### 4. Items by Location
*   **Forest Crossroads:** `flint`
*   **Stone Bridge:** `dry_wood`
*   **Forgotten Path:** `wild_herbs`
*   **Hut Interior:** `cauldron`, `empty_bottle`
*   **Echoing Cave:** `iron_key`
*   **Castle Keep:** `artifact`

### 5. Item Combinations
*   **Required:** `flint` + `dry_wood` → **Result:** `torch`
    *   *Purpose:* Necessary to provide light to enter the `Echoing Cave`.
*   **Required:** `wild_herbs` + `cauldron` → **Result:** `purifying_potion`
    *   *Purpose:* Used to cleanse the magical barrier at the `Castle Gate`.

### 6. Locked Paths & Conditions
*   **Echoing Cave:** Blocked by darkness. Requires `torch` to enter.
*   **Hut Interior:** Door is locked. Requires `iron_key` (found in the cave) to open.
*   **Castle Keep:** Blocked by a magical seal at the `Castle Gate`. Requires `purifying_potion` to break.

### 7. Level Objective & Resolution Flow
1.  **Start** at the `Forest Crossroads`. Pick up the `flint`.
2.  Go to the `Stone Bridge` and pick up the `dry_wood`.
3.  **Combine** `flint` and `dry_wood` to create a `torch`.
4.  Go to the `Forgotten Path`, pick up `wild_herbs`, and use the `torch` to enter the `Echoing Cave`.
5.  Find the `iron_key` inside the cave.
6.  Go to the `Hut Exterior` and use the `iron_key` to enter the `Hut Interior`.
7.  Pick up the `cauldron` and **combine** it with `wild_herbs` to brew the `purifying_potion`.
8.  Travel through the `Stone Bridge` to the `Castle Gate`.
9.  Use the `purifying_potion` to break the seal and enter the `Castle Keep`.
10. Retrieve the `artifact` and lift the curse.

### 8. Final world.yaml
Soubor `world.yaml` byl aktualizován podle tohoto návrhu.

```yaml
nodes:
  start:
    id: start
    description: "You stand at the Forest Crossroads. The ancient trees seem to whisper in a language long forgotten. To the north, a stone bridge leads over a dried-up creek. To the west, a narrow path disappears into deep shadows. To the east, you see the silhouette of an old hut."
    attributes: {}
    edges:
      - target_id: bridge
        label: "Cross the Stone Bridge (North)"
        conditions: []
      - target_id: forgotten_path
        label: "Follow the Forgotten Path (West)"
        conditions: []
      - target_id: hut_exterior
        label: "Approach the Old Hut (East)"
        conditions: []
    items:
      - flint
  bridge:
    id: bridge
    description: "The Stone Bridge is cracked and covered in silver moss. It leads to the ruins of what once was a grand castle gateway. The air feels heavy with magic here."
    attributes: {}
    edges:
      - target_id: start
        label: "Return to the Crossroads"
        conditions: []
      - target_id: castle_gate
        label: "Enter the Ruins (North)"
        conditions: []
    items:
      - dry_wood
  forgotten_path:
    id: forgotten_path
    description: "The Forgotten Path is overgrown with thorny vines. It leads to the mouth of a dark, Echoing Cave. You can hear the sound of dripping water from within."
    attributes: {}
    edges:
      - target_id: start
        label: "Back to the Crossroads"
        conditions: []
      - target_id: cave
        label: "Enter the Echoing Cave"
        conditions:
          - !HasItem torch
    items:
      - wild_herbs
  hut_exterior:
    id: hut_exterior
    description: "You are standing outside the Old Hut. The door is locked tight. A small wooden bench sits by the entrance."
    attributes: {}
    edges:
      - target_id: start
        label: "Return to the Crossroads"
        conditions: []
      - target_id: hut_interior
        label: "Unlock and enter the Hut"
        conditions:
          - !HasItem iron_key
    items: []
  hut_interior:
    id: hut_interior
    description: "The inside of the hut is dusty but preserved. Herbs hang from the ceiling, and an iron cauldron sits in the fireplace. There's an old recipe book on the table."
    attributes: {}
    edges:
      - target_id: hut_exterior
        label: "Leave the Hut"
        conditions: []
    items:
      - cauldron
      - empty_bottle
  cave:
    id: cave
    description: "The Echoing Cave is cold and damp. Your torch illuminates ancient carvings on the walls. In a small crevice, you find something metallic."
    attributes: {}
    edges:
      - target_id: forgotten_path
        label: "Exit the Cave"
        conditions: []
    items:
      - iron_key
  castle_gate:
    id: castle_gate
    description: "You stand before the Medieval Castle Gate. It is sealed by a shimmering magical barrier. A stone pedestal with a circular indentation sits nearby."
    attributes: {}
    edges:
      - target_id: bridge
        label: "Return to the Bridge"
        conditions: []
      - target_id: castle_keep
        label: "Use the Potion to break the seal"
        conditions:
          - !HasItem purifying_potion
    items: []
  castle_keep:
    id: castle_keep
    description: "The barrier shatters! You enter the Castle Keep. The forest's curse begins to lift as the sunlight pierces through the ancient halls. You have succeeded!"
    attributes: {}
    edges: []
    items:
      - artifact
items:
  flint:
    id: flint
    name: "Sharp Flint"
    description: "A piece of flint capable of striking sparks."
    can_pickup: true
  dry_wood:
    id: dry_wood
    name: "Bundle of Dry Wood"
    description: "Perfect for making a fire or a torch."
    can_pickup: true
  torch:
    id: torch
    name: "Bright Torch"
    description: "A makeshift torch providing light in the darkness."
    can_pickup: true
  wild_herbs:
    id: wild_herbs
    name: "Mystical Herbs"
    description: "Rare herbs with potent magical properties."
    can_pickup: true
  iron_key:
    id: iron_key
    name: "Old Iron Key"
    description: "A heavy, rusted key. It looks like it belongs to a door."
    can_pickup: true
  cauldron:
    id: cauldron
    name: "Iron Cauldron"
    description: "A sturdy cauldron for brewing potions."
    can_pickup: true
  empty_bottle:
    id: empty_bottle
    name: "Empty Glass Bottle"
    description: "A clear bottle for holding liquids."
    can_pickup: true
  purifying_potion:
    id: purifying_potion
    name: "Purifying Potion"
    description: "A glowing blue liquid that can cleanse magical barriers."
    can_pickup: true
  artifact:
    id: artifact
    name: "Heart of the Forest"
    description: "The ancient artifact that keeps the forest alive. You've found it!"
    can_pickup: true
combinations:
  - item1: flint
    item2: dry_wood
    result: torch
  - item1: wild_herbs
    item2: cauldron
    result: purifying_potion
```