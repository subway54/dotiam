# PLAN_05.2: Implementace herních příkazů a lokalizace světa

Tento plán popisuje kroky pro implementaci rozšířené sady herních příkazů a přechod na angličtinu v definici světa podle Zadání 5.2.

## 1. Cíle
- **Lokalizace:** Převést `world.yaml` do angličtiny.
- **Rozšířená sada příkazů:** Implementovat kompletní logiku pro h, l, g, x, c, p, d, i, u.
- **Interaktivní svět:** Rozšířit `world.yaml` tak, aby obsahoval předměty, kombinace a zamčené cesty pro demonstraci všech příkazů.

## 2. Technické kroky

### A. Rozšíření Datového Modelu (`dotiam-core`)
1.  **Aktualizace `GameAction`:**
    - Přidat varianty pro všechny nové příkazy:
        - `Help`
        - `Look`
        - `Go(String)`
        - `Explore(Option<String>)`
        - `Combine(String, String)`
        - `Pickup(String)`
        - `Drop(String)`
        - `Inventory`
        - `Use(String)`
2.  **Aktualizace `Node` a `Item`:**
    - Zajistit, aby `Node` mohl obsahovat předměty (items).
    - Definovat strukturu `Item` (id, description, can_pickup, atd.).
3.  **Logika kombinování:**
    - Přidat do `World` nebo `Node` definice možných kombinací předmětů.

### B. Implementace Logiky Příkazů (`dotiam-core`)
1.  **`parse_command`:**
    - Rozšířit parser o podporu aliasů (h, l, g, x, c, p, d, i, u).
    - Zvládnout parametry (např. "pickup sword", "use key", "combine flint steel").
2.  **`apply_action`:**
    - Implementovat logiku pro každý nový příkaz:
        - `Pickup/Drop`: Přesun předmětů mezi inventářem hráče a aktuálním uzlem.
        - `Look/Explore`: Detailnější popisy scény nebo konkrétních věcí.
        - `Inventory`: Výpis obsahu hráčova batohu.
        - `Use/Combine`: Změna stavu světa nebo inventáře (např. vytvoření nového předmětu).

### C. Lokalizace a Rozšíření Světa (`world.yaml`)
1.  **Překlad:** Přejmenovat ID uzlů a popisy do angličtiny (start -> start, forest -> forest, atd.).
2.  **Nový obsah:**
    - Přidat více místností (uzlů).
    - Přidat předměty na zem.
    - Přidat zamčené cesty vyžadující klíč (použití `Condition`).
    - Přidat předměty, které lze kombinovat (např. "stick" + "stone" = "hammer").

### D. Uživatelské Rozhraní (`dotiam-web`)
1.  **Zobrazení logu:** Zajistit, aby výstupy všech nových příkazů byly správně zobrazeny v herním logu.
2.  **UI pro inventář:** Možnost zobrazit inventář v postranním panelu nebo pomocí příkazu.

## 3. Harmonogram prací

1.  **Krok 1: Lokalizace `world.yaml`**
    - Kompletní přepis stávajícího světa do angličtiny.
2.  **Krok 2: Rozšíření `GameAction` a `apply_action`**
    - Implementace základních příkazů (help, look, inventory, pickup, drop).
3.  **Krok 3: Komplexní interakce (use, combine, explore)**
    - Implementace logiky pro interakci s předměty a jejich kombinování.
4.  **Krok 4: Testování a Demo**
    - Vytvoření komplexního `world.yaml`, který provede hráče všemi mechanikami.

## 4. Akceptační kritéria
- Všechny příkazy (h, l, g, x, c, p, d, i, u) fungují a mají odpovídající odezvu v logu.
- `world.yaml` je kompletně v angličtině.
- Hráč může sebrat předmět, vidět ho v inventáři, použít ho k odemknutí cesty nebo ho zkombinovat s jiným.
- Příkaz `help` vypíše srozumitelný seznam všech dostupných příkazů.
