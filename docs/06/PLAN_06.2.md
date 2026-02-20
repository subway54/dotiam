# PLAN_06.2: Implementace Mystické lesní adventury

Tento plán popisuje kroky pro implementaci nové úrovně "Whispering Woods" podle návrhu v Zadání 6.2 v `README_06.md`.

## 1. Cíle
- **Vytvoření herního světa:** Implementace 8 unikátních uzlů (lokací) s propojením podle grafu.
- **Implementace předmětů:** Přidání 9 předmětů s popisy a vlastnostmi.
- **Logika kombinací:** Definování 2 klíčových kombinací (pochodeň, lektvar).
- **Nastavení podmínek:** Implementace zamčených cest vyžadujících předměty nebo kombinace.
- **Lokalizace:** Celá úroveň bude v angličtině podle Zadání 6.1 a 6.2.

## 2. Technické kroky

### A. Definice Uzlů (Nodes) v `world.yaml`
1.  **Forest Crossroads (start):** Výchozí bod, obsahuje `flint`. Propojení na `bridge`, `forgotten_path`, `hut_exterior`.
2.  **Stone Bridge:** Spojka k hradu, obsahuje `dry_wood`.
3.  **Forgotten Path:** Cesta k jeskyni, obsahuje `wild_herbs`.
4.  **Hut Exterior:** Vstup k chatrči, zamčeno podmínkou `!HasItem iron_key`.
5.  **Hut Interior:** Alchymistická dílna, obsahuje `cauldron` a `empty_bottle`.
6.  **Echoing Cave:** Temná jeskyně, vstup vyžaduje `!HasItem torch`. Obsahuje `iron_key`.
7.  **Castle Gate:** Hradní brána, zamčena magickou bariérou, vyžaduje `!HasItem purifying_potion`.
8.  **Castle Keep:** Cíl hry, obsahuje `artifact`.

### B. Definice Předmětů (Items)
- `flint`: Základní materiál.
- `dry_wood`: Základní materiál.
- `torch`: Výsledek kombinace `flint` + `dry_wood`.
- `wild_herbs`: Surovina pro lektvar.
- `iron_key`: Klíč k chatrči.
- `cauldron`: Nástroj pro vaření lektvaru.
- `empty_bottle`: Nádoba (možná pro budoucí rozšíření, v návrhu 6.2 je součástí lokace).
- `purifying_potion`: Výsledek kombinace `wild_herbs` + `cauldron`.
- `artifact`: Cílový předmět.

### C. Definice Kombinací (Combinations)
1.  `flint` + `dry_wood` = `torch` (umožňuje vstup do jeskyně).
2.  `wild_herbs` + `cauldron` = `purifying_potion` (umožňuje vstup do hradu).

### D. Ověření a Integrace
1.  Aktualizace souboru `world.yaml` s novým obsahem.
2.  Spuštění testů `cargo test -p dotiam-core` pro ověření správné deserializace YAML a základní logiky.
3.  Ruční ověření průchodnosti (logický průchod: Crossroads -> Bridge -> Craft Torch -> Cave -> Key -> Hut -> Craft Potion -> Gate -> Keep).

## 3. Návrh YAML souboru (z README_06.md)

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

## 4. Akceptační kritéria
- `world.yaml` obsahuje všechny lokace, předměty a kombinace z návrhu.
- Hra je hratelná od začátku do konce bez záseků (softlocků).
- Všechny podmínky (`conditions`) u hran odpovídají hernímu postupu.
- Testy v `dotiam-core` procházejí.
