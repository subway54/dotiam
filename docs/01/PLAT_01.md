# Plán realizace PLAT_01: Architektura Rust Workspace

Tento dokument definuje strukturu Rust projektu pro hru Dotiam na základě specifikace v `README_01.md`. Projekt bude organizován jako Rust Workspace se třemi hlavními moduly (crates).

## 1. Struktura Workspace

Projekt bude využívat Rust workspace pro oddělení zodpovědností a umožnění sdílení kódu.

### Moduly (Crates):

1.  **`dotiam-core`** (Library)
    *   **Účel**: Obsahuje čistou doménovou logiku, herní pravidla, datové modely a stav hry.
    *   **Charakteristika**: Nezávislý na I/O (DB, Web). Musí být kompilovatelný i pro WASM (pro budoucí použití na frontendu, pokud bude potřeba).
    *   **Klíčové součásti**:
        *   `GameState`: Reprezentace stavu hráče a světa.
        *   `Actions`: Definice možných akcí hráče (Enum).
        *   `Engine`: Logika pro aplikaci akcí na stav (`apply(action, state) -> result`).
        *   `Models`: Entity jako `Item`, `Tile`, `Player`, `Encounter`.

2.  **`dotiam-app`** (Library / Binary)
    *   **Účel**: Implementace aplikační logiky, persistence a orchestrace herních běhů.
    *   **Závislosti**: `dotiam-core`.
    *   **Klíčové součásti**:
        *   `Repository`: Abstrakce pro ukládání/načítání (SQLite přes `sqlx`).
        *   `Services`: Správa herních relací (GameRuns), persistence snapshotů a event logů.
        *   `Config`: Načítání statického obsahu hry (definice itemů, mapy z TOML/JSON).

3.  **`dotiam-web`** (Binary)
    *   **Účel**: Webový server postavený na `axum`, který poskytuje HTMX rozhraní.
    *   **Závislosti**: `dotiam-app`, `dotiam-core`.
    *   **Klíčové součásti**:
        *   `Handlers`: Zpracování HTTP požadavků, volání `dotiam-app` služeb.
        *   `Templates`: Renderování HTML fragmentů pomocí `askama` nebo `tera`.
        *   `Assets`: Statické soubory (CSS, obrázky).
        *   `HTMX Integration`: Logika pro partial updates.

## 2. Sdílení kódu (`dotiam-core`)

Modul `dotiam-core` bude sdílen mezi:
*   **Backendem (`dotiam-web` / `dotiam-app`)**: Pro validaci tahů a výpočet stavu na serveru.
*   **Frontendem (`dotiam-web` / WASM)**: V budoucnu může být `dotiam-core` kompilován do WebAssembly pro client-side predikce nebo offline logiku, aniž by se musela duplikovat pravidla hry.

## 3. Postup implementace

1.  **Inicializace Workspace**:
    *   Vytvoření kořenového `Cargo.toml` definujícího členy workspace.
2.  **Vytvoření modulů**:
    *   `cargo new --lib dotiam-core`
    *   `cargo new --lib dotiam-app`
    *   `cargo new dotiam-web`
3.  **Definice domény v `dotiam-core`**:
    *   Základní structy pro `Player`, `World`, `Tile`.
    *   Základní enum `GameAction`.
4.  **Implementace persistence v `dotiam-app`**:
    *   Nastavení `sqlx` a migrací pro SQLite.
    *   Logika pro ukládání a načítání JSON snapshotů.
5.  **Webový server v `dotiam-web`**:
    *   Zprovoznění Axum serveru.
    *   První endpointy pro zobrazení stavu a odeslání akce.

## 4. Technologický stack (rekapitulace)

*   **Language**: Rust
*   **Web Framework**: Axum
*   **Templating**: Askama (typově bezpečná)
*   **Database**: SQLite + SQLx
*   **Frontend**: HTMX + CSS
*   **Serialization**: Serde (pro JSON snapshoty a komunikaci)
