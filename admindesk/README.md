# AdminDesk — pomoc zdalna A.D.M.I.N

AdminDesk to wersja [RustDesk](https://github.com/rustdesk/rustdesk) z marką A.D.M.I.N (4dmin.pl).
Licencja: AGPL-3.0, tak jak oryginał — kod źródłowy tej wersji jest w tym repozytorium.

## Co zmieniamy względem RustDesk
- nazwa programu: `src/common.rs` → `load_custom_client()` (`APP_NAME = "AdminDesk"`),
- ikony i logo: `python admindesk/brand.py` generuje je z `admindesk/grafika/*.svg`,
- metadane EXE: `flutter/windows/runner/Runner.rc`,
- build Windows: `.github/workflows/admindesk.yml` (push na gałąź `admindesk` → artefakt; tag `admindesk-*` → wydanie).

## Aktualizacja z RustDesk
```bash
git fetch upstream && git merge upstream/master   # na gałęzi admindesk
```
