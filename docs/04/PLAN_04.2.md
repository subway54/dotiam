# PLAN_04.2: Vylepšení ASCII Editoru (Ovládání a Rozměry)

Tento plán popisuje kroky pro implementaci vylepšení ASCII editoru podle Zadání 04.2, včetně fixní velikosti mapy a interaktivního ovládání pomocí klávesnice.

## 1. Cíle
- **Fixní rozměr:** Mapa v editoru bude mít 255x255 znaků.
- **Startovní pozice:** Výchozí pozice kurzoru bude uprostřed (127, 127).
- **Klávesové ovládání:** 
    - Pohyb kurzoru šipkami.
    - Změna typu dlaždice klávesami `+`, `-`, `|`, ` ` (mezera).
    - Funkce "zpět" klávesou `z`.
- **Optimalizace vykreslování:** Vzhledem k velikosti 255x255 bude potřeba vykreslovat pouze výřez (viewport) kolem kurzoru, aby prohlížeč nebyl přetížen.

## 2. Technické kroky

### A. Backend (`dotiam-web`)
1.  **Úprava Editor View:**
    - Změnit logiku generování mřížky v `editor_handler`.
    - Místo bounding boxu existujících dlaždic definovat viewport (např. 21x21 nebo 31x31 znaků) centrovaný na aktuální pozici kurzoru.
    - Zajistit, aby souřadnice v editoru odpovídaly celkovému rozsahu 255x255.
2.  **API pro "Zpět" (Undo):**
    - Implementovat jednoduchou historii (např. uložení předchozího stavu dlaždice v session nebo dočasné tabulce) pro klávesu `z`.

### B. Frontend a Šablony (`dotiam-web`)
1.  **JavaScript pro klávesové zkratky:**
    - Přidat `EventListener` na `keydown` v `editor.html`.
    - Mapovat šipky na navigaci (přesměrování nebo HTMX load na nové souřadnice).
    - Mapovat `+`, `-`, `|`, ` ` na volání endpointů pro změnu typu dlaždice.
    - Mapovat `z` na volání undo endpointu.
2.  **Viewport Rendering:**
    - Upravit šablonu `editor.html`, aby zobrazovala pouze výřez mapy.

### C. Datový Model (`dotiam-core`)
1.  **Konstanty:** Definovat `MAP_SIZE = 255` a výchozí pozici.

## 3. Iterace vývoje

1.  **Fáze 1: Viewport a fixní rozsah**
    - Implementace omezení pohybu kurzoru na 0-254.
    - Implementace výřezu (viewportu) v backendu, aby se nepřenášelo 65k buněk najednou.
2.  **Fáze 2: Klávesové ovládání**
    - Přidání JS logiky pro zachytávání kláves.
    - Propojení s existujícími HTMX endpointy.
3.  **Fáze 3: Funkce Undo**
    - Implementace logiky pro klávesu `z`.

## 4. Akceptační kritéria
- Kurzor se pohybuje v rozsahu 0-254 na obou osách.
- Stisknutí klávesy `+`, `-`, `|` nebo `Space` okamžitě změní typ dlaždice na pozici kurzoru.
- Stisknutí `z` vrátí předchozí typ dlaždice.
- Mapa se plynule posouvá (viewport následuje kurzor).
