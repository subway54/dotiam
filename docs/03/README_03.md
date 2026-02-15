# Dotia 03

## Zadání 03

jak pohodlně vytvořit data pro takovou hru?

## Řešení 03

Nejkomfortnější je udělat si **datový model, který je zároveň “autorovatelný”** (dobře se píše a čte) a **strojově validovatelný** (aby se ti hra nerozbila kvůli překlepu). U tebe (web + Rust) se velmi osvědčily 3 přístupy – od nejjednoduššího po nejškálovatelnější.

---

## 1) Nejrychlejší start: YAML/JSON soubory v repozitáři

**Proč:** edituješ v editoru, verzování v Gitu, jednoduché pro první iterace.

### Co ukládat

* `rooms` / `nodes` (místnosti nebo kvadranty)
* `edges` (pohyby a podmínky)
* `items` (předměty)
* `events` (skripty/trigger)
* `texts` (lokalizace, popisy)

### Doporučený tvar (YAML – čitelné pro člověka)

```yaml
rooms:
  - id: r_start
    title: "Chodba"
    view:
      forward: "corridor_01"
      left: "corner_left_01"
      right: "corner_right_01"
      back: "corridor_back_01"
    text:
      short: "Studená chodba, slabý průvan."
      detail: "Zdi jsou vlhké. Na zemi jsou staré stopy."
    exits:
      forward: r_2
      left: r_deadend
items:
  - id: key_rusty
    name: "Rezavý klíč"
    tags: ["key"]
```

### V Rustu

* `serde` + `serde_yaml` / `serde_json`
* validace po načtení (unikátní ID, existence referencí)

Tohle tě dostane do hratelného prototypu nejrychleji.

---

## 2) Pohodlné „tvoření obsahu“: jednoduchý editor přímo ve hře (dev mód)

**Proč:** Nejrychlejší tvorba mapy a popisů v praxi je *kliknout, napsat, uložit*.

### Jak by to fungovalo

* speciální „Editor mode“ (jen pro tebe)
* klikneš na „Upravit scénu“
* měníš:

    * texty
    * obrázkový „view“ (výběr assetu)
    * exits (kam vede vpřed/vlevo/vpravo)
    * předměty ve scéně
* uloží se to jako JSON/YAML

**Výhoda:** rychlost iterace, neustálé testování.
**Nevýhoda:** musíš napsat editor UI (ale může být fakt minimalistický).

---

## 3) Nejlepší dlouhodobě: „content pipeline“ (zdrojové soubory → build → hotová data)

**Proč:** jakmile přibydou podmínky, questy, kombinace itemů, začne být YAML ruční peklo.

### Pipeline

1. **Zdroj**: YAML + „skriptování“ (viz níž)
2. **Build krok**:

    * validace
    * expandování šablon (např. generování koridorů)
    * kompilace do jednoho binárního balíku (nebo JSON)
3. **Runtime**: hra načte jen hotová data

### Co ti to umožní

* generovat opakující se struktury (chodby, lesní cesty)
* mít „prefaby“ místností
* dělat lint/CI (chyby se chytí při buildu)

---

# Klíčový trik: dělej data „uzlově“, ne “v mřížce”

Ty mluvíš o kvadrantech, ale UI je first-person. To znamená, že hráč reálně vnímá:

* jsem v uzlu
* mám 0–4 směry
* každý směr vede do dalšího uzlu a má podmínky

To se mnohem líp autoruje než X/Y grid.

Takže interně klidně drž:

* `node_id`
* `exits: {forward, left, right, back}`
* `state flags` (dveře otevřeny, most spadl…)

Grid je super pro pathfinding; pro tvorbu obsahu je často horší.

---

# Jak řešit logiku bez “programování v datech”

Nejhorší je, když začneš psát v YAML „podmínkové stromky“ ručně.

Dvě pohodlné možnosti:

## A) Jednoduché podmínky jako data

```yaml
exits:
  forward:
    to: r_gate
    require:
      has_item: key_rusty
    on_fail: "Dveře jsou zamčené."
```

## B) Lehký „skript“ jako DSL (mini jazyk)

Např. textové akce:

* `set_flag gate_open`
* `if has_item(key) then ... else ...`

Tohle jde udělat minimalisticky a robustně.

---

# Doporučený „workflow“ pro tebe (Rust + web)

1. Start: YAML soubory v repu
2. Napiš loader + validátor (serde + kontrola referencí)
3. Přidej dev mód „edit & save“
4. Až to poroste: build pipeline (validace + generace)

---

## Co bych ti doporučil hned teď jako minimum

* IDčka všude (stringy)
* separátně `rooms.yaml`, `items.yaml`, `events.yaml`
* validace:

    * unikátní ID
    * každý exit míří na existující room
    * item reference existuje
    * žádné cykly? (nebo naopak cykly povolit)

---

Když mi řekneš, jestli chceš mapu **ručně navrženou** nebo **částečně procedurální** (chodby/les jako generované segmenty), navrhnu ti konkrétní datový formát a validace přesně pro ten styl.


## Zadání 03 a

pokud se rozhodnu pro edit mod, tak co bude výstupem takového editoru, kam se data uloží. teď mám POC hry ale data jsou nyní pouze naseedovaná v sqlite v memory.

## Řešení 03 a

Když máš **edit mód běžící ve webu** (a single-player), máš v zásadě 4 rozumné cíle, kam editor ukládá data. V praxi se často kombinuje **“pracovní úložiště”** + **“export”**.

Níže ti dám varianty a pak doporučený setup pro tvůj stav (POC + sqlite in-memory).

---

## 1) Výstup editoru jako soubory (JSON/YAML) + export/import

**Co je výstup:** jeden nebo více souborů, např. `world.json`, `items.json`, `events.json` (nebo `world.yaml`).

**Kam se uloží během editace (ve webu):**

* **IndexedDB** (nejběžnější a nejpraktičtější) nebo
* `localStorage` (jen pro drobnosti; má limity)

**Jak se dostanou “ven”:**

* tlačítko **Export** → stáhne se soubor do počítače (download)
* tlačítko **Import** → nahraješ soubor zpět do editoru

✅ Výhody:

* jednoduché, čitelné, verzovatelné v Gitu
* snadno se validuje a diffuje
* nezávislé na SQLite implementaci ve WASM

❌ Nevýhody:

* runtime v POC máš v SQLite → budeš dělat “load JSON → naplnit DB”, nebo DB úplně vyhodíš

---

## 2) Výstup editoru je “SQLite soubor” (persistovaná DB)

**Co je výstup:** `game_content.sqlite` – normální databázový soubor.

**Kam se uloží během editace (ve webu):**

* **OPFS (Origin Private File System)** přes File System Access API / WasmFS (nejlepší “disk” v prohlížeči)
* alternativně ukládání do **IndexedDB** jako blob

**Jak se dostane ven:**

* Export DB jako soubor ke stažení

✅ Výhody:

* skoro žádná změna proti tomu, co máš (jen přestane být in-memory)
* DB umí referenční integritu, indexy, dotazy

❌ Nevýhody:

* v prohlížeči je “SQLite soubor” někdy opruz podle stacku (záleží, jaký sqlite wasm používáš)
* hůř se verzují změny (diffy)

---

## 3) Hybrid: Editor ukládá do “kanonického formátu” (JSON/YAML), hra si z toho při buildu vyrobí SQLite

To je z dlouhodobého pohledu nejpohodlnější.

**Co je výstup editoru:** JSON/YAML (kanonický zdroj pravdy)

**Během editace:** IndexedDB

**Pro release:** build krok `pack-content`:

* validace
* vygeneruje se `content.sqlite` nebo `content.bin/json` bundle

✅ Výhody:

* editor je snadný a přenosný
* release runtime je rychlý (SQLite / binární pack)
* přirozeně se dělá CI validace

❌ Nevýhody:

* malý “build krok” navíc (ale stojí to za to)

---

## 4) Ukládání na server (API)

Tohle bych u single player POC nedělal jako první.

✅ Výhody: spolupráce, zálohy
❌ Nevýhody: musíš řešit auth, hosting, API, konflikty, atd.

---

# Co doporučuju pro tvůj stav (POC + sqlite in-memory)

Nejmenší bolest:

### A) Editor ukládá průběžně do IndexedDB (pracovní kopie)

* autosave každých pár sekund / při změně
* rychlé, robustní, offline

### B) Editor má Export/Import jako **jeden `world.json`**

* exportuješ kdykoliv a dáš do Gitu
* importem si kdykoliv obnovíš stav

### C) Hra při startu: `world.json` → “seed do SQLite”

Takže **SQLite zůstane** (klidně i in-memory), ale seed nebude hardcoded, jen se načte z JSON a naplní DB.

> Tohle ti umožní mít pořád SQL dotazy a současně pohodlný content workflow.

---

## Jak může vypadat “world.json” (praktický minimal)

* `schema_version`
* `rooms`
* `items`
* `events`
* `strings` (popisy)

A hlavně všude IDčka a reference.

---

## Kdy dává smysl přejít na persistovanou SQLite (varianta 2)

Až:

* budeš chtít “jedním souborem” přenášet vše (včetně třeba cache assetů)
* nebo chceš v editoru dělat pokročilé dotazy/filtry (např. “ukaž všechny místnosti bez popisu”)

---

# Doporučená “technická kostra” editoru

* **In-memory model** (Rust structs) je zdroj pro UI
* **Serializer**: `model -> JSON`
* **Deserializer**: `JSON -> model`
* **Storage adapter**:

    * `IndexedDbStore.save(json)`
    * `IndexedDbStore.load()`

A extra:

* `export()` vrátí JSON blob ke stažení
* `import(file)` načte JSON, validuje, uloží do IndexedDB

---

## Validace, kterou fakt chceš hned od začátku

* unikátní `id`
* každý `exit.to` existuje
* item reference existuje
* schema version odpovídá (migrace)

---

Pokud mi napíšeš, jak teď v POC vypadá tvoje SQLite struktura (tabulky + pár sloupců), navrhnu ti:

1. konkrétní JSON schema (aby se dobře exportovalo),
2. mapování `JSON → SQLite seed`,
3. a jak to celé udělat tak, aby ses nezamotal do migrací.
