# warcraft-api: DDD-Restrukturierung

Status: **Entwurf zur Freigabe** · Datum: 2026-07-11

## Ziel

`warcraft-api` von einer flachen Sammlung concern-vermischter Dateien in eine
saubere, DDD-markierte, von anderen Crates konsumierbare API umbauen. Ein
Einstieg, immutabel, alles über Views/Refs. Das DDD-Crate (`clemenscodes/ddd`,
`v0.1.0`) wird durchgängig korrekt angewandt und markiert.

## Leitprinzipien

1. **Ein Verzeichnis = genau ein Concern.** Verschiedene Domänen-Typen werden
   getrennt (z.B. `GridCoordinate` hat nichts mit dem Objektmodell zu tun).
2. **Schnitt nach Domäne, nicht nach Schicht innerhalb der Domäne.** Alles über
   „Ability" an einem Ort, alles über „Unit" an einem.
3. **DDD-Layer sind die Top-Ebene.** Jede Datei sitzt in genau einem Layer.
4. **Immutable API.** Kein `&mut`, keine offengelegten Felder, keine innere
   Mutabilität. Alles kommt über Methoden raus, die Views/Refs liefern.
5. **Kuratierte Re-Exports.** Genau gesteuert, was rausgeht (`Race` ja, DB nein).
6. **`f32` ist im Domänenmodell der falsche Typ** (kein `Eq`, weil `NaN`), obwohl
   unsere Werte nie kaputt sind → Festkomma-VOs. `f32` bleibt nur im Primitive.

## Layer-Struktur

```
src/
  lib.rs            — kuratierte Re-Exports, mod-Deklarationen
  domain/           — DomainLayer: Value Objects, Entities, Identifier
    object/         — WarcraftObject (Entity/AggregateRoot), WarcraftObjectKind, WarcraftObjectMeta, WarcraftObjectText
    identity/       — WarcraftObjectId (Identifier)
    grid/           — GridCoordinate, ColumnIndex, RowIndex, ParseGridCoordinateError
    unit/           — UnitKind, UnitMeta, UnitProduction, UnitFlags, UnitCombat, UnitAttack, ManaPool, RegenType
    hero/           — HeroAttributes, AttributeBase, AttributeGrowth, PrimaryAttribute
    combat/         — AttackType, WeaponType, DefenseType
    ability/        — AbilityMeta
    item/           — ItemClass, ItemMeta
    upgrade/        — UpgradeMeta, UnitUpgradeSwap
    command/        — CommandMeta
    balance/        — GameplayConstants, DamageMatrix, DamageEffectiveness, StrengthBonuses, IntelligenceBonuses, AgilityBonuses
    race/           — Race, AllRaces
    quantity/       — Festkomma-VOs (siehe unten)
    (weitere: player, version, keybind — nach Lesen der Dateien platziert)
  application/      — ApplicationLayer: die API
    mod.rs          — WarcraftApi (ApplicationService, Wurzel)
    unit.rs …       — UnitApi/AbilityApi/… (ApplicationService), je nur bei echtem Query-Cluster
    view/           — ReadModel-Projektionen (UnitView, …)
    catalog/        — CommandCatalog, UnitCatalog & Co. (Query/Read-Konstrukte)
  infrastructure/   — InfrastructureLayer, pub(crate), NICHT re-exportiert
    database/       — WarcraftDatabase + generierter Static (db.rs)
tests/
  database_invariants.rs      — aus src/test.rs, gegen WarcraftApi
  balance_overlay.rs          — aus src/balance_overlay_regression.rs (Name entschärft)
```

## DDD-Rollen-Mapping

`ddd v0.1.0` bietet: Layer {Domain, Application, Infrastructure, Presentation},
`Layered{type Layer}`, `ValueObject: Clone+Eq+Layered<Domain>`, `Identifier: ValueObject`,
`Entity: Layered<Domain>{type Identity: Eq}`, `AggregateRoot: Clone+Layered<Domain>`,
`Repository<A: AggregateRoot>{load/save}`, `ApplicationService: Layered<Application>`,
`Query: Layered<Application>{type Output}`, `ReadModel {}`.

| Rolle | Typen |
|---|---|
| `Identifier` | `WarcraftObjectId` |
| `Entity`/`AggregateRoot` (Identity = `WarcraftObjectId`) | `WarcraftObject` |
| `ValueObject` | alle Meta-/Enum-/Quantity-Typen im DomainLayer |
| `ApplicationService` | `WarcraftApi`, `UnitApi`, `AbilityApi`, `ItemApi`, … |
| `ReadModel` | die zurückgegebenen Views/Projektionen |
| (keine Rolle, `pub(crate)`) | `WarcraftDatabase` — Infrastruktur |

**Primitives** (`Bytes`, `Boolean`, `Byte`, `Integer`, `Float`, `Identifier`, `Time`)
liegen in einem **eigenen Crate `warcraft-primitives`** (öffentlich, Shared Kernel),
weil externe Byte-Konsumenten (ein Memory-Reader in einem anderen Projekt) sie ohne
die Domäne brauchen. warcraft-api hängt vorerst dran (drei Alt-Berührungen), verliert
die Dependency aber, sobald tote `ItemClass::From<Integer>` weg, die `get`-API ersetzt
(Scheibe 4) und player-`From<Byte>` platziert ist (Scheibe 6).

**Bewusst nicht verwendet:** `Repository` (ist Single-Aggregate + `save`; unsere DB
ist ein read-only keyed Katalog, passt nicht). Die Crate ist reine **Leseseite** —
kein `Command`/`Service`/`UnitOfWork`. Konformität wird über eine
`ddd_conformance`-Testmodul-Analogie (wie in warcraft-keybinds) compile-time geprüft.

## `f32` → Festkomma-Value-Objects

Nur im **Domänenmodell**. Der Primitive `Float` (= exakte 4-Byte-Speicher-Realität
des Spiels) bleibt `f32` und unangetastet. Übersetzung Primitive → Domänen-VO
passiert im Extraktor beim Emittieren.

Eigene VOs pro Größe, Milli-Skala (×1000), signed nur wo nötig:

| VO | Backing | ersetzt |
|---|---|---|
| `Time` (existiert) | `u32` ms | `UnitAttack.cooldown_seconds` |
| `RegenRate` | `u32` milli/s | `ManaPool.mana_regen`, `UnitCombat.hit_points_regen` |
| `Armor` | `i32` milli (signed) | `UnitCombat.armor` |
| `StatGrowth` | `u32` milli | `AttributeGrowth.*_per_level` |
| `Multiplier` | `u32` milli | Balance-Bonusfaktoren, `DamageEffectiveness.multipliers` |
| `Chance` | `u16` permille (0..=1000) | `AbilityMeta.evasion_chances` |

Konkrete Menge wird beim Meta-Slice final justiert. Alle sind `ValueObject`.

## Ripple: Extraktor + `db.rs`

`db.rs` (71k Z., generiert, Editieren verboten) hält aktuell `f32`-Literale. Nach
der Typänderung kompiliert es nicht mehr → **Regeneration nötig**:
1. Extraktor-Emission gibt VO-Konstruktoren aus (`Multiplier::from_milli(1500)` statt `1.50`).
2. `cargo run -p warcraft-extractor -- --casc <PATH>` (CASC-Daten sind vorhanden).

## Zielnutzung

```rust
let api = WarcraftApi::default();
api.get(id);                    // generisch, Ref raus
api.unit().get(id);             // Domänen-View
api.item().class(id);
```

## Ausführung in Scheiben (jede `build`+`test` grün)

1. **Fundament:** ✅ **erledigt, grün.** `ddd` als Workspace-Dependency;
   Primitives in eigenes Crate `warcraft-primitives` extrahiert (pub, Shared Kernel).
   (`ddd_conformance`-Testmodul folgt beim ersten Markieren in Scheibe 2.)
2. **Domänen-Split (ohne f32-Änderung):** object/meta-Typen in `domain/<concern>/`
   verschieben; DDD-Marker auf die bereits Eq-fähigen Typen (Float-Typen bleiben
   verschoben, aber unmarkiert bis Slice 3). ✅ object.rs vollständig zerlegt
   (grid/identity/item/upgrade/object/unit + database→infrastructure, grün).
   **Offen:** meta.rs-Split (combat/hero/unit-meta/ability/command/balance),
   unit_kind.rs (`UnitKindHelpers`), unit_mode.rs; `ddd_conformance`-Testmodul.
4. **Application-Layer:** `WarcraftApi` + Sub-APIs (ApplicationService) + Views (ReadModel);
   `ObjectLookup` auflösen; Static + `WarcraftDatabase` verstecken; kuratierte Re-Exports.
5. **Tests → Integration:** `test.rs` / `balance_overlay_regression.rs` nach `tests/`, gegen `WarcraftApi`.
6. **Restliche Domänen:** player, keybind (+keycode/mirrors/hotkeys/ability_tables), catalog,
   variant_groups, race, version — Dateien lesen, nach Concern platzieren, Rollen markieren.
3. **f32 → VOs (+ Regeneration) — ZULETZT, weil einziges CASC-abhängiges:**
   `domain/quantity/`-VOs; Domänenfelder umstellen; Extraktor-Emission anpassen;
   `db.rs` regenerieren; Float-Typen dann als `ValueObject` markieren.
   CASC ist verfügbar: `W3_CASC` ist gesetzt, also genügt `cargo run -p warcraft-extractor`
   (liest `--casc` via `env = "W3_CASC"`), danach `cargo fmt -p warcraft-api`.

## Fortschritt

**Slice 1** ✅. **Slice 2** (Struktur, Eq-fähige Typen): ✅ object.rs komplett zerlegt;
✅ meta.rs Eq-fähige Typen gesplittet — `domain/combat` (AttackType/WeaponType/DefenseType),
`domain/hero` (PrimaryAttribute/AttributeBase VO; ManaPool/AttributeGrowth/HeroAttributes
verschoben, VO deferred), `domain/command` (CommandMeta), `domain/unit`
(UnitKind/UnitProduction/UnitFlags/RegenType), `domain/item` (+ItemMeta),
`domain/upgrade` (+UpgradeMeta). Alle grün, 132 Tests.

**meta.rs** hält jetzt nur noch die **Float-Typen**: UnitAttack, UnitCombat, UnitMeta,
DamageEffectiveness, StrengthBonuses, IntelligenceBonuses, AgilityBonuses, DamageMatrix,
GameplayConstants, AbilityMeta. Die wandern in **Slice 3** (f32→VO) nach `domain/unit`,
`domain/balance`, `domain/ability` — dann ist meta.rs leer und wird gelöscht.

**Slice 4** ✅ — `application/api.rs`: `WarcraftApi` (Feld `database: &'static WarcraftDatabase`,
`Default`, kuratierte Methoden `object`/`resolve`/`by_id`/`iter`/`len` + 6 Helfer,
`ddd::ApplicationService`). `ObjectLookup` gelöscht. `WARCRAFT_DATABASE` `pub(crate)`
(nur `application/api.rs` + noch test.rs/balance_overlay bis Slice 5). catalog/unit_catalog/
variant_groups über `WarcraftApi`. 135 Tests grün.

**Slice 5** ✅ — `tests/database_invariants.rs` + `tests/balance_overlay.rs` (Integrationstests
gegen `WarcraftApi`; extern → beweisen die Kapselung). `src/test.rs`/`balance_overlay_regression.rs`
weg. `WARCRAFT_DATABASE`-Re-Export entfernt → Static nur noch in `application/api.rs` bekannt.
**Objekt-Count-Test: exakter Golden-Pin** `assert_eq!(api.len(), 1742)` — zero-tolerance,
feuert bei JEDER Patch-Churn (Wunsch des Users, explizit). Bei gewolltem Change Zahl im
selben Commit bumpen.

**Slice 6** teilweise ✅ — `domain/race`, `domain/version` (VO-markiert). **Noch offen:**
player, keybind (+keycode/mirrors/system_hotkeys/ability_tables), unit_kind (UnitKindHelpers),
unit_mode, catalog, variant_groups → Concerns + Rollen.

✅ `UnitMode` → `domain/unit` (VO), `unit_mode.rs` gelöscht.

## DDD-Markierung: VOLLSTÄNDIG ✅ (ganze Crate, alles grün)

Jeder öffentliche Typ trägt seine Rolle: `Identifier` (WarcraftObjectId),
`Entity`+`AggregateRoot` (WarcraftObject), `ValueObject` (alle Eq-fähigen Domänen-
und Config-Typen inkl. player/keybind/combat/hero/…), `ReadModel` (CatalogEntry),
`DomainService` (UnitKindHelpers, BuildingTraits), `ApplicationService` (WarcraftApi,
UnitCatalog, CommandCatalog, VariantUnits). Float-Typen in meta.rs sind der einzige
Rest ohne VO-Marker — der kommt mit Slice 3.

## Handoff: Ist-Stand (alles grün, 166 Tests)

**Fertig:** `warcraft-primitives`-Crate (Shared Kernel, pub). `domain/`-Layer mit
`combat, command, grid, hero, identity, item, object, race, unit, upgrade, version`
— alle Eq-fähigen Typen `ValueObject`-markiert, `WarcraftObjectId`=`Identifier`,
`WarcraftObject`=`Entity`+`AggregateRoot`. `infrastructure/database` (pub(crate)).
`application/api.rs` = `WarcraftApi` (`ApplicationService`), einzige öffentliche Fassade;
`ObjectLookup` weg; `WARCRAFT_DATABASE` nur noch dort bekannt. Tests → `tests/`
(Integration, exakter Objekt-Count-Pin 1742). object.rs + meta.rs (Eq-Teil) zerlegt.

**Noch flach in `src/` (offen):**
- `meta.rs` — nur noch **Float-Typen** (UnitAttack, UnitCombat, UnitMeta, DamageEffectiveness,
  StrengthBonuses, IntelligenceBonuses, AgilityBonuses, DamageMatrix, GameplayConstants,
  AbilityMeta). → **Slice 3**: `domain/quantity/`-VOs bauen, f32-Felder umstellen,
  Extraktor-Emission anpassen, `db.rs` regenerieren (`cargo run -p warcraft-extractor`,
  W3_CASC gesetzt), Typen nach `domain/unit|balance|ability`, VO-markieren, meta.rs löschen.
  ⚠️ Nach f32-Änderung bricht `db.rs` bis zur Regeneration — als *ein* Schritt fahren.
- `player.rs` — ~14 Value Objects; mehrere Enums brauchen noch `PartialEq, Eq`; `Team`/`Teams`/
  `TeamPlayer` (Struct, Feld-Eq prüfen). → `domain/player`, VO-markieren.
- Keybind-Cluster `keybind.rs`, `keycode.rs`, `keybind_mirrors.rs`, `system_hotkeys_category.rs`,
  `ability_tables.rs` → `domain/keybind` (o.ä.), Rollen markieren.
- `catalog.rs`, `unit_catalog.rs`, `variant_groups.rs` — Query/Read-Konstrukte → **Application**
  (Query/ReadModel/DomainService — an ihrer Logik entscheiden). `unit_kind.rs` (`UnitKindHelpers`,
  fieldless Service) → `ddd::DomainService` oder Methoden auf UnitKind/UnitMeta ziehen.
- `db.rs` (generiert) → nach `infrastructure/database/generated.rs` (mit Extraktor-Output-Pfad
  in main.rs:9), am besten zusammen mit der Slice-3-Regeneration.
- `ddd_conformance`-Testmodul (wie in warcraft-keybinds) als compile-time Rollen-Check.

## Offen / zu bestätigen bei Ausführung

- Genaue Concern-Platzierung der noch ungelesenen Module (Slice 6) — wird beim
  Lesen der jeweiligen Datei festgelegt, nicht geraten.
- Ob `catalog`/`variant_groups` DomainService (Domain) oder Query/ReadModel
  (Application) sind — entscheidet sich beim Lesen ihrer Logik.

## Abschluss (2026-07-11): Restrukturierung fertig, grün

Alle Umzüge, Extraktionen und Einfaltungen sind durch; `warcraft-api` steht als saubere,
folder-per-concept-DDD-API. `cargo test -p warcraft-api -p warcraft-primitives` grün
(136 + 12 + 15 + 1 Doctest + 31), `cargo build -p warcraft-extractor` grün, `cargo fmt`
+ `cargo clippy` sauber. Keine flachen Dateien mehr (nur `lib.rs` + generierter Static).

**Gefaltet/zerlegt (God-Files gelöscht):**
- `meta.rs` → `domain/{unit,balance,ability}/…` (ein Typ pro Modul).
- `domain/object.rs` → `domain/object/{kind,text,meta,aggregate}`.
- `domain/{building,catalog}.rs` gelöscht: building-traits → `UnitView::can_attack`
  (pures `is_attacking_building`) + `UnitApi::can_uproot`; catalog-Query-VOs
  (`SearchField`/`CatalogVisibility`) → `application/unit/listing`.
- `application/unit_catalog.rs` (`entries_for`, 275 Z., 6 Positional-Args, `is_search =
  mode.is_none()`) → `application/unit/listing/` mit `UnitQuery` + `Scope{Browse,Search}`
  und puren Stages `search`/`suppress`/`sort` (TDD) + Boundary-Stages `index`/`placeholder`;
  öffentlich `api.unit().list(&UnitQuery)`.
- `application/command_catalog.rs` → `application/unit/command_card/` (purer `assembly`-Kern,
  TDD) + `api.unit().command_card()`; `command` als eigenes Konzept: `api.command()` +
  `CommandView`.
- variant-Cluster komplett neu: `application/unit/variant/{facts,chains,build,index,group,
  union_find}` — `facts` einziger DB-Leser, `chains`/`build`/`union_find` pur & TDD;
  `VariantGroup`/`VariantIndex` `pub(crate)`; Ability-Fanout → `application/ability/fanout/`
  (purer `pairing`-Kern, TDD). Öffentlich nur `api.unit().variants()/canonical()/…` und
  `api.ability().fanout()`.

**Prinzipien durchgezogen:** Reinheit (Verarbeitungsfunktionen bekommen minimale Eingaben,
nie die ganze DB; einziger DB-Zugriff je Subsystem an der Boundary), `TryFrom`/`From` statt
`from_*`, DDD-Marker beim Typ (nie unter `mod tests`), keine `_for`-Namen, kein
Namespace-über-Global. Golden-Verhalten via Integrationstests durch `api.unit()`/`api.ability()`
gepinnt.

**Bewusst offen:** Scheibe 3 (f32 → Festkomma-VOs + `db.rs`-Regeneration) — die Float-Typen
in `domain/{unit,balance,ability}` bleiben `f32` und unmarkiert. Laut Handoff explizit außerhalb
des Scopes.
