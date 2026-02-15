# PLAN_04: ASCII Map Editor a Editor Popisů

Tento plán popisuje kroky pro implementaci nové stránky editoru, která umožní vizuální editaci mapy pomocí ASCII znaků a úpravu textových popisů jednotlivých lokací.

## 1. Cíle
- **ASCII Mapa:** Zobrazení herního světa pomocí znaků `+`, `-`, `|` a mezer.
- **Interaktivní Kurzor:** Možnost pohybu po mapě a výběru konkrétní dlaždice.
- **Editor Popisů:** Textové pole pro úpravu popisu scény na aktuální pozici kurzoru.
- **Náhled Obrázku:** Zobrazení obrázku přiřazeného k dané pozici.

## 2. Technické kroky

### A. Rozšíření Datového Modelu (`dotiam-core`)
1.  **Mapování ASCII:** Přidat logiku pro převod mezi `TileType` a ASCII znaky:
    - `+`: Rozcestí (průchodné všemi směry - stávající typy jako Plains/Forest/atd. lze reprezentovat takto, pokud nemají omezení).
    - `-`: Východ-Západ.
    - `|`: Sever-Jih.
    - ` `: Neprůchodné (prázdné místo).
2.  **Logika Mapy:** Metoda pro vygenerování 2D pole znaků reprezentující aktuální známý svět (bounding box existujících dlaždic).

### B. Backend API (`dotiam-web`)
1.  **Nový Endpoint:** `/game/{id}/editor` - hlavní stránka nového editoru.
2.  **API pro aktualizaci popisu:** Endpoint pro uložení změněného textu popisu pro danou pozici.
3.  **API pro změnu typu dlaždice:** Endpoint pro změnu znaku v mapě (přepsání typu dlaždice).

### C. Frontend a Šablony (`dotiam-web`)
1.  **Šablona `editor.html`:**
    - Levý panel: ASCII mapa v `<pre>` bloku nebo mřížce.
    - Pravý panel: 
        - `<textarea>` pro editaci popisu.
        - `<img>` tag pro náhled scény.
2.  **Interaktivita (HTMX/JS):**
    - Kliknutí na znak v mapě přesune "kurzor" a načte popis a obrázek dané lokace do pravého panelu.
    - Změna v textovém editoru se automaticky (nebo tlačítkem) uloží do DB.

## 3. Iterace vývoje

1.  **Fáze 1: ASCII Rendering**
    - Implementace výpočtu mřížky mapy v `dotiam-core`.
    - Vytvoření základní stránky `/editor` se statickým zobrazením mapy.
2.  **Fáze 2: Editor Popisů**
    - Propojení výběru v mapě s načítáním dat lokace.
    - Implementace ukládání popisu scény.
3.  **Fáze 3: Úprava struktury mapy**
    - Možnost měnit znaky přímo v ASCII náhledu (přepsání typu dlaždice).

## 4. Akceptační kritéria
- Stránka `/editor` zobrazuje mapu složenou z definovaných ASCII znaků.
- Výběr lokace v mapě aktualizuje zobrazený popis a obrázek.
- Změna popisu v editoru se trvale uloží a projeví se i v herním módu.
- Mapu lze rozšiřovat/měnit zápisem ASCII znaků (pokud bude implementováno v editačním poli).
