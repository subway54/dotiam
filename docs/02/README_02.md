# dotiam 02

## Zadání 02

potřebuji vymyslet jak bude vypadat obrazovka v této hře. Chci něco extrémně jednoduchého na ovládání a zároveň trochu originálního, resp. nechci kopírovat, ale zase nechci něco úplně mimo zvyklosti. Sám bych si představoval

- jeden obrázek zobrazující co vidím v první osobě. Pokud mám možnost jít dopředu tak vidím chodbu pokud vlevo nebo vpravo tak vidím roh chodby vlevo nebo vpravo. Pokud jsme na konci chodby tak vidím třeba jen stěnu nebo řadu stromů, nebo prostě konec chodníku.

Taky nějaký popis scény, nebo nějaké poznámky.

To co nevím, jestli mít na webové stránce něco jako ovladač chození: šipka dopředu, dozadu, vlevo, vpravo.

Pak asi něco jako obsah kapes, batoh nebo něco takového, a taky vlastnosti mé osoby.

Napadá mě jestli nezobrazovat vždy jen obrázek a pod ním jednu z těchto možností:

- ovládání (tlačítka)
- popis scény
- inventář
- vlastnosti mé bytosti

Taky nevím jestli zobrazovat vše

## Návrhy řešení 02

Jo, tohle zadání je **překvapivě čisté** – máš už skoro hotovou filozofii UI 👍
Zkusím ti to trochu **učesat**, nabídnout **2–3 jasné varianty obrazovky** a hlavně odpovědět na otázku *„zobrazovat vše, nebo přepínat?“*.

---

## Základní princip (který bych držel)

* **1 hlavní obraz** = co *vidím teď* (first-person, statický pohled)
* **minimum trvalých prvků**
* **jedna akce = jedno rozhodnutí**
* žádné HUD peklo, žádné mini-mapy, žádné lišty přes půl obrazovky

Tohle je strašně dobrý základ pro textově-obrazovou adventuru.

---

## Varianta A: „Obraz + kontextový panel“ (doporučuju jako default)

![Image](https://learn.arcade.academy/en/latest/_images/castle_01.png)

![Image](https://media.pxlart.com/d6bdfa8a-9bfc-4ba8-a5c0-b45e28c74885-thumbnail.jpg)

![Image](https://miro.medium.com/v2/resize%3Afit%3A1400/1%2AnGFE8fijjr6kTC11p6336w.jpeg)

![Image](https://miro.medium.com/v2/resize%3Afit%3A1400/1%2ATs3hh5wD3EFlZ8Vqh4r50A.jpeg)

### Rozložení obrazovky

```
┌───────────────────────────┐
│                           │
│       HLAVNÍ OBRAZ        │  ← 70–80 % výšky
│   (chodba / roh / les)    │
│                           │
└───────────────────────────┘
┌───────────────────────────┐
│  Kontextový panel         │
│  (přepínatelný obsah)     │
└───────────────────────────┘
```

### Kontextový panel (JEN JEDEN VIDITELNÝ)

Pod obrázkem **vždy jen jedna „vrstva“**, přepínaná ikonami nebo klávesami:

* 🧭 **Akce / pohyb**
* 📜 **Popis scény**
* 🎒 **Inventář**
* ❤️ **Vlastnosti**

Tohle je klíčové:
👉 *hráč se nikdy nedívá na všechno najednou*

---

### 🧭 Pohyb – bez „herního ovladače“

Nedělal bych klasický kříž šipek. To už je klišé.

Místo toho:

```
[ JÍT VPŘED ]
[ OTOČIT SE VLEVO ]   [ OTOČIT SE VPRAVO ]
[ VRÁTIT SE ]
```

nebo ještě víc „textově“:

```
→ Jít vpřed
← Otočit se vlevo
→ Otočit se vpravo
↩ Vrátit se
```

**Důvod:**

* odpovídá textovým adventurám
* jde snadno rozšířit o akce typu „prozkoumat“, „sebrat“, „otevřít“

---

### 📜 Popis scény

Krátký. Fakt krátký.

> *Chodba je úzká a studená. Ze severu vane slabý průvan.
> Na zemi jsou staré stopy.*

Max 2–3 věty.
Žádné romány. Atmosféra > informace.

---

### 🎒 Inventář

Ne grid, ne ikony všude.

Spíš seznam:

```
• Rezavý klíč
• Plátěný vak (3/5)
• Starý nůž
```

Kliknutím:

* **použít**
* **zkombinovat**
* **zahodit**

---

### ❤️ Vlastnosti

Opět minimalismus:

```
Zdraví: ████░
Síla:   ██░░░
Vnímání: ███░░
```

Žádné číselné tabulky – to si můžeš schovat do „detailu“.

---

## Varianta B: „Čistý obraz + textový vstup“ (odvážnější, víc oldschool)

![Image](https://www.museumofplay.org/app/uploads/2021/08/OpeningZork.jpg)

![Image](https://market-resized.envatousercontent.com/previews/files/272274837/preview.jpg?cf_fit=crop\&crop=top\&format=auto\&h=590\&q=85\&s=8b54d098edfb36af4f1f88d0ef3af5396f62752ae135a82d95b8c06bc04b3807\&w=590)

![Image](https://i.imgur.com/JDDTMB5.jpg)

### Rozložení

```
┌───────────────────────────┐
│                           │
│        OBRAZ              │
│                           │
└───────────────────────────┘
> _
```

Hráč píše:

* `jít`
* `vlevo`
* `inventář`
* `prozkoumat`

💡 **Tip:**
Nabízej *našeptávač*:

```
> j_
  jít
  jít vpřed
```

Tohle je:

* extrémně jednoduché
* extrémně levné na UI
* extrémně „jiné“, ale pořád pochopitelné

---

## Varianta C: „Všechno vždy vidět“ – proč bych to NEDĚLAL

Tohle je přesně ta past.

Problémy:

* hráč skenuje UI místo obrazu
* atmosféra jde pryč
* UI začne bobtnat (přidáš jeden prvek → už se to veze)

**Výjimka:**
mobilní verze → dole malá lišta s ikonami (🧭 📜 🎒 ❤️)

---

## Shrnutí – moje doporučení (TL;DR)

👉 **Jdi cestou přepínaného panelu pod obrazem**

* obraz = král
* vždy jen **jedna** informační vrstva
* žádný klasický gamepad
* textové volby místo ikon tam, kde to jde

Pokud chceš, v dalším kroku můžeme:

* navrhnout **konkrétní HTML strukturu**
* nebo **stavový automat UI (Rust + WASM)**
* nebo udělat **wireframe v ASCII / pseudo-kódu**

Stačí říct, kam chceš jít dál 👌
