# PLAN_02: Implementace herního rozhraní (Obraz + Text)

Tento dokument definuje kroky potřebné pro implementaci uživatelského rozhraní podle "Varianty B" z `README_02.md`. Cílem je vytvořit minimalistické rozhraní, kde dominantou je obraz scény, doplněný o textový popis a interaktivní textový vstup.

## 1. Architektura UI
Rozhraní bude rozděleno na tři hlavní části:
- **Obrazová část:** Centrální prvek zobrazující aktuální scénu (first-person pohled).
- **Textový výstup:** Oblast pro zobrazení popisu scény, výsledků akcí a historie.
- **Textový vstup:** Interaktivní řádek (prompt) pro zadávání příkazů s našeptávačem.

## 2. Technické kroky

### A. HTML/CSS Struktura (`dotiam-web`)
1.  **Layout:** Vytvořit hlavní kontejner, který udrží obraz a textovou oblast v harmonickém poměru.
2.  **Obraz:** Implementovat responsivní prvek pro obrázek (s podporou pro placeholder, pokud obrázek chybí).
3.  **Terminál:** Stylizovat oblast pro text tak, aby vypadala jako klasická adventura (monospace font, barvy, scrollback).
4.  **Prompt:** Vytvořit input field s prefixem `> ` a stylem, který nenarušuje atmosféru.

### B. Herní logika a Parser (`dotiam-core`)
1.  **Command Parser:** Rozšířit jádro o parser, který dokáže zpracovat textové příkazy (`jít`, `vlevo`, `prozkoumat`, `inventář`).
2.  **Stavový automat:** Implementovat logiku přechodů mezi scénami na základě příkazů.
3.  **Registr příkazů:** Definovat seznam dostupných příkazů pro každou scénu (pro potřeby našeptávače).

### C. WASM Integrace (`dotiam-web/src`)
1.  **Event Handling:** Zachytávání stisku kláves v inputu (Enter pro odeslání, šipky pro historii/našeptávač).
2.  **Rendering Loop:** Aktualizace obrazu a textu v DOMu při změně stavu hry.
3.  **Našeptávač (Autocomplete):** Implementovat logiku, která na základě rozepsaného slova nabízí validní pokračování příkazu.

## 3. Iterace vývoje

1.  **Prototyp 0.1:** Statické HTML s obrázkem a inputem, který vypisuje zadaný text do konzole.
2.  **Prototyp 0.2:** Propojení s Rust jádrem (WASM), základní pohyb pomocí textových příkazů (`jít`).
3.  **Prototyp 0.3:** Přidání textového popisu scény a historie zpráv.
4.  **Prototyp 0.4:** Implementace našeptávače a vylepšení vizuální stránky (CSS animace přechodů).

## 4. Akceptační kritéria
- Hráč vidí obrázek odpovídající jeho poloze a směru.
- Pod obrázkem je textový popis aktuální situace.
- Hráč může psát příkazy a ty se vykonávají.
- Rozhraní je přehledné a nerozptyluje zbytečnými prvky.
