# PLAN_03: Edit mód, YAML Export a Inicializace

Tento plán popisuje kroky pro implementaci editačního režimu, možnost exportu herního světa do YAML souboru a automatickou inicializaci SQLite databáze z tohoto souboru při startu aplikace.

## 1. Cíle
- **Edit mód:** Webové rozhraní umožňující měnit vlastnosti dlaždic (tile type) a popisy přímo v prohlížeči.
- **YAML Export:** Tlačítko pro stažení/uložení aktuálního stavu světa do souboru `world.yaml`.
- **Boot-up Load:** Při startu aplikace načíst `world.yaml` (pokud existuje) a použít jej jako výchozí stav pro novou hru místo hardcodovaného seudu.

## 2. Technické kroky

### A. Datový model a Serializace (`dotiam-core`)
1.  **YAML podpora:** Přidat závislost `serde_yaml` do `dotiam-core`.
2.  **Definice World Template:** Vytvořit strukturu `WorldTemplate`, která bude obsahovat pouze statická data světa (mapu dlaždic, jejich typy a popisy), odděleně od dynamického stavu hráče.
3.  **Implementace metod:**
    - `WorldTemplate::to_yaml() -> String`
    - `WorldTemplate::from_yaml(content: &str) -> Result<Self, ...>`

### B. Úložiště a Inicializace (`dotiam-app`)
1.  **Seeding z YAML:** Upravit `Repository::create_run` (nebo přidat novou metodu), která přijme `WorldTemplate` a inicializuje `GameState`.
2.  **File I/O:** Přidat logiku pro čtení `world.yaml` z kořenového adresáře projektu při inicializaci aplikace.

### C. Webové rozhraní (`dotiam-web`)
1.  **Edit Mód UI:**
    - Přidat přepínač (toggle) mezi "Play Mode" a "Edit Mode".
    - V Edit módu zobrazit formuláře pro úpravu aktuální dlaždice (změna typu: Forest, Plains, atd.).
    - Implementovat HTMX endpointy pro aktualizaci světa v DB.
2.  **Export tlačítko:**
    - Přidat tlačítko "Export World to YAML".
    - Vytvořit endpoint `/game/{id}/export`, který vrátí YAML obsah.
3.  **Persistence:** Zajistit, aby změny v Edit módu byly okamžitě ukládány do SQLite.

## 3. Iterace vývoje

1.  **Fáze 1: YAML a Seedování**
    - Implementace `WorldTemplate` a podpora YAML.
    - Úprava startu aplikace: pokud existuje `world.yaml`, použít ho pro start nové hry.
2.  **Fáze 2: Export**
    - Přidání tlačítka v UI, které vygeneruje a nabídne ke stažení `world.yaml`.
3.  **Fáze 3: Edit Mód**
    - Základní editační prvky v UI (změna typu dlaždice na aktuální pozici).
    - Vizuální odlišení Edit módu od herního módu.

## 4. Akceptační kritéria
- Je možné změnit typ dlaždice v Edit módu a změna se projeví po návratu do Play módu.
- Kliknutí na Export vygeneruje platný YAML soubor.
- Pokud je v projektu soubor `world.yaml`, nově vytvořená hra obsahuje data z tohoto souboru.
