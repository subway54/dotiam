# PLAN_04.3: Nové typy dlaždic a vylepšení vizualizace

Tento plán popisuje kroky pro implementaci změn v ASCII mapě podle Zadání 04.3, včetně nových typů terénu, jejich klávesových zkratek a vizuálního odlišení v editoru.

## 1. Cíle
- **Nové typy dlaždic:**
    - `.` : Planina (Plains)
    - `^` : Hory (Mountains, neprůchozí)
    - `~` : Voda (Water)
    - `S` : Start (Start)
    - `G` : Brána (Gate)
    - `#` : Zeď (Wall)
- **Vylepšené ovládání:** Přidání klávesových zkratek pro tyto nové typy.
- **Vizuální styl:** Neprázdné dlaždice v editoru budou mít tmavě šedé pozadí.

## 2. Technické kroky

### A. Datový Model (`dotiam-core`)
1.  **Rozšíření `TileType`:**
    - Upravit enum `TileType` v `dotiam-core/src/lib.rs`.
    - Ponechat stávající typy (Forest, Ruins, Cave, atd.) nebo je konsolidovat podle nového zadání, pokud je to žádoucí (zadání explicitně uvádí nové znaky, které se částečně překrývají s existujícími jako Plains).
2.  **Mapování znaků:**
    - Aktualizovat metodu `World::get_ascii_map` pro podporu nových znaků.
3.  **Popisy lokací:**
    - Aktualizovat `GameState::get_current_description` pro nové typy terénu.
4.  **Průchodnost (volitelně):**
    - Implementovat logiku, kde hory (`^`) jsou neprůchozí pro hráče.

### B. Backend API (`dotiam-web`)
1.  **Mapování v Editoru:**
    - Aktualizovat `editor_handler` v `dotiam-web/src/main.rs`, aby vracel správné znaky pro `EditorTile`.
2.  **Endpointy pro aktualizaci:**
    - Rozšířit `quick_update_handler` a `update_description_handler` pro podporu nových typů dlaždic.

### C. Frontend a Šablony (`dotiam-web`)
1.  **Styling v `editor.html`:**
    - Upravit CSS třídu `.map-char.exists` (nebo přidat novou), aby měla tmavě šedé pozadí (`#222` nebo podobné).
2.  **Klávesové zkratky v `editor.html`:**
    - Aktualizovat JavaScriptový `EventListener`, aby reagoval na nové klávesy:
        - `.` -> Plains
        - `^` -> Mountains
        - `~` -> Water
        - `S` -> Start
        - `G` -> Gate
        - `#` -> Wall

## 3. Iterace vývoje

1.  **Fáze 1: Model a Backend**
    - Úprava `TileType` a logiky renderingu v `dotiam-core`.
    - Úprava handlerů v `dotiam-web`.
2.  **Fáze 2: Frontend (JS/CSS)**
    - Implementace klávesových zkratek a změna barev v editoru.
3.  **Fáze 3: Popisy a Logika**
    - Doplnění výchozích textů pro nové typy a případná kontrola průchodnosti.

## 4. Akceptační kritéria
- Všechny nové znaky (., ^, ~, S, G, #) lze vkládat pomocí klávesnice v editoru.
- Mapa v editoru správně zobrazuje tyto znaky.
- Dlaždice, které nejsou prázdné, mají v editoru tmavě šedé pozadí.
- Hra (Play mode) správně interpretuje nové typy dlaždic (zobrazuje odpovídající popisy).
