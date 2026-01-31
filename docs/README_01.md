# dotia

## Zadání

cílem projektu je vytvořit herní platformu, resp. hru.

Vlastnosti:

- bude se hrát jako single player na kterékoliv platformě kde je web
- bude signle playerová, tahová ale umožní v pozdějších fázích spolupráci hráčů, například sdílením speciální tokenů
- přemýšlím nad tím, že bude hráč interaktivně pracovat se hrou pomocí příkazů (buď tlačítka, různé volby, nebo například i psaný text)
- to co uživatel uvidí bude nějaká jednoduchý obraz a text a ovládací prvky, půjde zřejmě o takovou jednoduchou adventuru
- hráč bude získávat speciální předměty, bude je moci kombinovat a používat
- hráč se bude vlastně pohybovat po mapě v kvadrantech


potřebuji vymyslet:
1) specifikaci rozharní
2) způsob uložení vlastností hráče, prostředí mapy
3) návrh infrastruktury a architektury (preferuji rust a htmx)

## Řešení

Níže máš jeden konzistentní návrh, který ti pokryje UI/specifikaci, ukládání stavu a infrastrukturu/architekturu pro **single-player tahovou webovou adventuru** s mapou v kvadrantech – s tím, že si necháš otevřená vrátka pro pozdější “kooperaci” přes sdílené tokeny.

---

## 1) Specifikace rozhraní (UI/UX)

### Základní principy

* **Jeden “herní pohled” = jeden tah** (request → server vyhodnotí → vrátí nový render).
* UI je “text + jednoduchý obraz + ovládání”.
* Vstup je **volba akcí** (tlačítka), plus volitelně **příkazový řádek** (text).
* Vše jde dělat přes **HTMX** partial updates (bez SPA).

### Layout (3 panely)

**A) Header / Status bar**

* Jméno postavy, HP/energie, čas/tah, lokace (x,y), aktuální “biom/typ políčka”
* Ikony/stavy (poisoned, bleeding, buffy, quest flagy)
* “Save/Load” (jen pro debug/admin nebo pro hráče jako menu)

**B) Main view**

1. **Scene Image**

    * jednoduchý obrázek pro typ lokace (např. ruina/les/jeskyně) + případné overlay ikonky (nepřítel, loot)
2. **Scene Text (log)**

    * poslední výstup hry + krátká historie (např. posledních 10 zpráv)
    * důležité události zvýraznit (loot, damage, quest)

**C) Action panel**

* **Primární akce** kontextové pro lokaci:

    * Pohyb: N/E/S/W (a případně diagonály později)
    * Interakce: prozkoumat, sebrat, použít, kombinovat, mluvit, otevřít, zapálit…
* **Sekundární akce / Inventory**

    * inventář jako seznam itemů (klik → detail → “Use / Combine / Drop”)
    * “Combine mode”: vybereš item A a B → server zkusí recept
* **Command input** (volitelné)

    * textové pole: `go north`, `use key`, `combine herb bottle`, `inspect statue`
    * autocomplete může být později; na start stačí parser s pár příkazy

### HTMX flow (konkrétně)

* Každé tlačítko dělá `POST /game/{run_id}/action` s payloadem např. `{"type":"Move","dir":"N"}`.
* Server vrátí HTML fragmenty:

    * `#scene` (obrázek + text)
    * `#actions` (nové akce)
    * `#status` (staty)
    * `#inventory`
* Výhoda: žádné složité front-end stavy, celý “state of truth” je na serveru.

### Minimální “UI specifikace” (co se musí renderovat vždy)

* `StatusBarViewModel`
* `SceneViewModel` (image_id, text, entries log)
* `ActionListViewModel` (list of actions, disabled/enabled, hotkeys)
* `InventoryViewModel` (items, selected_item, possible_combinations)
* `ModalViewModel` (volitelně: dialog, potvrzení, detail itemu)

---

## 2) Uložení vlastností hráče a prostředí mapy

Tady je dobré si ujasnit “co je run”:

* **GameRun** = jedna rozehraná hra (save slot).
* V singleplayeru může být run mapovaný na session/cookie, později na účet.

### Doporučený model: Event log + snapshot (praktický kompromis)

Nejčistší je event sourcing, ale nechceš se tím zdržet. Kompromis:

* **Snapshot** (rychlé načtení):

    * hráčské stats, inventář, pozice, seed RNG, quest flags
* **Event log** (audit a debug, snadné “replay”, pozdější kooperace):

    * tahy typu Move/Use/Combine/Inspect
    * výsledky: DamageDealt, ItemAdded, TileRevealed…

**Pro MVP** klidně jen snapshot, ale event log ti později strašně pomůže.

### Datový model (stručně)

**PlayerState**

* `id`
* `pos: (x,y)`
* `stats: hp, energy, level, …`
* `inventory: Vec<ItemStack>`
* `flags: HashSet<FlagId>`
* `cooldowns / timers` (pokud bude)

**WorldState**

* mapa je mřížka kvadrantů (tile)
* neukládej celou mapu jako obří JSON, pokud je generovaná

    * použij **seed + “diffs”**:

        * `world_seed`
        * `tile_overrides` jen tam, kde se něco změnilo (loot sebral, dveře otevřel, NPC odešel)
* pokud mapa není generovaná, může být “static content” v souborech a do DB dáš jen změny

**TileState**

* `tile_type` (forest/ruins/cave…)
* `discovered` (fog of war)
* `entities`: třeba `Vec<EntityId>` (enemy, chest, npc)
* `props`: drobné vlastnosti (is_open, is_burnt, etc.)

**Item**

* definice itemů je “content data” (JSON/TOML v repu)
* v inventáři ukládej jen `item_id`, `qty`, plus `instance_modifiers` (např. “požehnaný meč”)

### Databáze

Pro Rust web hru je super:

* **SQLite** pro začátek (jednoduché deploy, rychlé, spolehlivé).
* Až bude potřeba multiuser/škálování → **PostgreSQL** (stejný model, jen jiný driver).

**Tabulky (MVP)**

* `game_runs(id, created_at, updated_at, player_snapshot_json, world_snapshot_json, rng_state, version)`
* volitelně `events(id, run_id, turn, event_json, created_at)`

> Později pro sdílené tokeny mezi hráči: přidáš tabulku `shared_tokens(token_id, owner_run_id, payload, redeemed_by_run_id, expires_at)`.

---

## 3) Návrh infrastruktury a architektury (Rust + HTMX)

### Doporučený stack

* Web server: **axum** (nebo actix-web; axum je dnes hodně příjemný)
* Templating: **askama** (typově bezpečné šablony) nebo **tera**
* DB: **sqlx** (async, compile-time query checks)
* Session: cookie session nebo server-side session (např. Redis později)
* Static assets: jednoduché obrázky + CSS (Tailwind volitelně, ale klidně čisté CSS)

### Architektura: “Clean/DDD-ish”, ale prakticky

Rozděl to na 4 vrstvy:

1. **Domain (game rules)**

    * čistá logika, bez DB/web
    * `GameEngine::apply(action, state) -> (new_state, effects, view_model)`
    * typy: `Action`, `Effect`, `State`, `Tile`, `Item`, `Recipe`
    * determinismus přes RNG seed

2. **Application (use-cases)**

    * orchestrace: load run → apply action → persist → render
    * `PerformAction(run_id, action)`

3. **Infrastructure**

    * `RunRepository` (SQLite/Postgres)
    * `ContentRepository` (načítání definic itemů/tiles/quests z files)
    * `ImageService` (mapuje tile_type → image url)

4. **Interface (web)**

    * axum handlers
    * HTMX endpoints vrací HTML fragmenty

### Endpointy (MVP)

* `GET /` → landing / pokračovat / nový run
* `POST /run/new` → vytvoří run, redirect `/game/{id}`
* `GET /game/{id}` → full page render
* `POST /game/{id}/action` → vrací fragmenty (status/scene/actions/inventory)
* `POST /game/{id}/command` → parser textu → action → stejné jako action
* `GET /assets/...` statika

### Jak udělat “tahovost” a konzistenci

* Každý request nese `run_id` + `turn` (optimistický lock)
* V DB máš `version`/`turn` a update děláš jen pokud sedí
* Když nesedí, vrátíš “conflict” a klient refreshne (v singleplayeru se to stává jen při doubleclicku)

### Připravenost na budoucí spolupráci

I když teď singleplayer:

* drž akce jako **commands** a výsledky jako **events/effects**
* sdílení tokenu bude jen “import event”:

    * hráč A vygeneruje token → B ho redeemne → engine aplikuje `ReceiveToken(...)`

---

## Konkrétní návrh “MVP scope” (aby ses nezasekl)

1. Mapa 20×20, pár tile typů, fog of war
2. Pohyb N/E/S/W, prozkoumat, sebrat, inventář, použít
3. 5–10 itemů + 3 kombinace (recepty)
4. 2 typy encounterů (např. “náhodná událost” + “truhla”)
5. SQLite snapshot + jednoduchý log událostí (klidně jen append)

---

## Rychlé rozhodnutí: generovaná vs ručně definovaná mapa

* **Generovaná**: seed + pravidla + overrides → nejlepší pro škálování obsahu
* **Ručně definovaná**: rychlejší na kontrolu designu, ale víc dat

Praktický hybrid: ručně definuj “regiony” (biomy) a uvnitř generuj detaily.

---

Chceš, abych ti to rovnou sepsal jako:

* konkrétní **OpenAPI-ish** endpointy + payloady pro HTMX,
* návrh DB schématu (SQL) pro SQLite/Postgres,
* a kostru projektu v Rustu (moduly `domain/ application/ infra/ web/` + ukázka handleru `POST /action`)?
