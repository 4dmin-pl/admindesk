# AdminDesk — pomoc zdalna A.D.M.I.N

AdminDesk to wersja [RustDesk](https://github.com/rustdesk/rustdesk) z marką A.D.M.I.N (4dmin.pl).
Licencja: AGPL-3.0, tak jak oryginał — kod źródłowy tej wersji jest w tym repozytorium.

## Dwie aplikacje
| Rola (`ADMINDESK_ROLE`) | Nazwa | Dla kogo | Co potrafi |
|---|---|---|---|
| `agent` (domyślna) | AdminDesk | klient | tylko przyjmuje połączenia; ustawienia zablokowane; bez tunelu/terminala/kamery |
| `konsola` | AdminDesk-Konsola | technik A.D.M.I.N | tylko łączy się z Agentami |

## Co zmieniamy względem RustDesk
- `src/admindesk.rs` — nazwa, rola, blokady ustawień, (etap 2) serwer i klucz publiczny na sztywno;
  jedyny haczyk: `crate::admindesk::apply()` w `load_custom_client()` (`src/common.rs`),
- `src/platform/windows.rs` `get_license_from_exe_name()` — wyłączona konfiguracja serwera z nazwy pliku exe,
- ikony i logo: `python admindesk/brand.py` generuje je z `admindesk/grafika/*.svg`;
  limit wysokości logo 60→80 px w `flutter/lib/common.dart` (logo z podpisem),
- metadane EXE: `flutter/windows/runner/Runner.rc`, `Cargo.toml`, `libs/portable/Cargo.toml`,
- build Windows: `.github/workflows/admindesk.yml` (push na gałąź `admindesk` → artefakty; tag `admindesk-*` → wydanie).

## Aktualizacja z RustDesk
```bash
git fetch upstream && git merge upstream/master   # na gałęzi admindesk
```
