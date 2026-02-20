# Dotiam 04

## Zadání 04.1

Vytvoř novou stránku pro editaci herního plánu, ve které bude pomocí ascii znaků
zobrazena mapa.

Následující znaky budou představovat herní mapu:
* Space bude prázdné místo na které hráč nebude moci přistoupit
* znak plus bude rozcestí
* znak mínus bude pozice ze které lze jít pouze na východ nebo západ
* znak | bude pozice ze které lze jít pouze na sever nebo na jich

Vedle mapy bude také prostor ve kterém bude textový editor ve kterém bude možné
popsal scénu hry na pozici kde zrovna v mapě je kurzor.

Další část obrazu bude image který se v daném místě zobrazuje.

## Zadání 04.2

Základní velikost mapy bude 255x255 znaků.

První pozice bude uprostřed mapy.

Editor mapy se ovládá kurzorovými šipkami.

Pomocí následujících kláves bude modifikována pozice na které je kurzor.

* '+' - křižovatka
* '-' - pozice ze které lze jít pouze na východ nebo západ
* '|' - pozice ze které lze jít pouze na sever nebo na jich
* ' ' - smazání pozice
* 'z' - zpět původní hodnota

# Zadání 04.3

Změna mapy. Nově bude fungovat takto:

* '.' - planina ()
* '^' - hory (neprůchozí)
* '~' - voda
* 'S' - start
* 'G' - brána
* '#' - zeď

Uprav taky patřičně ovládání editoru.
Pozice kde není pouze prázdný znak bude mít
světleší barvu než černá jako podkladovou - tmavě šedá.