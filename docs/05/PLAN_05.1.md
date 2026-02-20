# PLAN_05.1: Přechod na grafovou reprezentaci světa

Tento plán popisuje kroky pro přepracování herního světa z mřížky ASCII znaků na strukturu grafu podle Zadání 5.1.

## 1. Cíle
- **Grafová struktura:** Svět bude definován jako graf uzlů (pozic) a hran (přechodů).
- **Uzly (Nodes):** Každý uzel bude mít kódové označení, atributy a popisy.
- **Hrany (Edges):** Přechody mezi uzly budou obsahovat podmínky (vlastnosti/předměty hráče), které musí být splněny pro průchod.
- **Nahrazení mřížky:** Logika pohybu se změní ze směrového pohybu (N, S, E, W) na výběr z dostupných přechodů.

## 2. Technické kroky

### A. Redesign Datového Modelu (`dotiam-core`)
1.  **Definice `Node`:**
    - `id: String` (kódové označení).
    - `attributes: HashMap<String, String>` (atributy pozice).
    - `description: String`.
2.  **Definice `Edge`:**
    - `target_id: String` (kam přechod vede).
    - `label: String` (název směru/akce, např. "Vstoupit do jeskyně").
    - `conditions: Vec<Condition>` (seznam podmínek pro přechod).
3.  **Definice `Condition`:**
    - Typy podmínek: `HasItem(String)`, `HasAttribute(String, String)`, `MinHP(u32)`.
4.  **Aktualizace `World`:**
    - Nahradit `HashMap<Position, Tile>` za `HashMap<String, Node>`.
5.  **Aktualizace `Player`:**
    - Změnit `pos: Position` na `current_node: String`.
    - Přidat `inventory: Vec<String>` a `attributes: HashMap<String, String>` pro vyhodnocování podmínek.

### B. Úprava Logiky Hry (`dotiam-core`)
1.  **Pohyb:**
    - `apply_action` bude místo směrů zpracovávat ID cílových uzlů nebo názvy akcí z hran.
    - Implementovat metodu `can_traverse(player, edge) -> bool`.
2.  **Příkazy:**
    - Dynamické generování dostupných příkazů na základě hran aktuálního uzlu.

### C. Úprava Úložiště a Serializace (`dotiam-app` & `dotiam-core`)
1.  **JSON/YAML Serializace:** Aktualizace `WorldTemplate` a `GameState` pro práci s grafem.
2.  **Databáze:** Upravit schéma (pokud je potřeba) nebo jen serializaci do JSONu v `game_runs`.

### D. Webové Rozhraní (`dotiam-web`)
1.  **Zobrazení:**
    - Místo ASCII mapy (která v grafu ztrácí přímý smysl) zobrazit seznam dostupných cest jako tlačítka/odkazy.
    - Upravit `partial_game.html` pro zobrazení možností pohybu.
2.  **Editor:**
    - Původní ASCII editor bude muset být nahrazen nebo výrazně upraven pro editaci grafu (např. formulář pro přidávání hran). *Poznámka: Zadání 5.1 se zaměřuje na model, editor může být řešen v další fázi.*

## 3. Iterace vývoje

1.  **Fáze 1: Nové datové struktury**
    - Implementace `Node`, `Edge` a `Condition` v `dotiam-core`.
    - Úprava `Player` a `GameState`.
2.  **Fáze 2: Logika průchodu a podmínek**
    - Implementace vyhodnocování podmínek.
    - Úprava `parse_command` pro dynamické příkazy.
3.  **Fáze 3: Webová integrace**
    - Aktualizace handlerů a šablon pro zobrazení grafových dat.

## 4. Akceptační kritéria
- Svět je reprezentován uzly a hranami místo mřížky.
- Hráč se může pohybovat pouze po definovaných hranách, pokud splňuje podmínky.
- Hra vypisuje dostupné cesty z aktuálního uzlu.
- YAML export/import funguje s novou strukturou.
