# nuclear
Rust library and tool for opening various graphics-related Nintendo DS files

## Roadmap

### Version 0.1 (#8CF)
- [x] **Core:** Support NCLR, NCGR, NSCR
- [x] **Core:** Project format with wrappers, allow loading and saving it
- [ ] **Core:** Import PNG into NSCR (smart/GRIT-like conversion)
- [ ] **GUI:** Load/create/save projects
    - [x] Load
    - [x] Create
    - [x] Save changes (from the tabs themselves)
    - [ ] Save as
- [x] **GUI:** Edit project metadata
- [x] **GUI:** Editors for the implemented formats
    - [x] NCLR
    - [x] NCGR
    - [x] NSCR
- [x] **GUI:** Editor features
    - [x] NCLR: import/export NCLR to a standardized palette format (?)
    - [x] NCLR: in-app palette edits, with preview options
    - [x] NSCR: import/export NCSR to .png
- [ ] **GUI:** Import Nintendo files into project
+ [x] **GUI:** Interactive sidebar
- [ ] **Mantainance:** Remove img::export, replace with convenience file creation functions

### Versions 0.2 (#F88) - 1.0 (#20F)
- [ ] **Core:** Import PNG into NSCR (smart/GRIT-like conversion)
- [ ] **Core**: NCER / NANR support
- [ ] **Core**: Portable project format
- [ ] **fission**: Get a basic version of the framework
    - this objective will be detailed more in the future
- [ ] **GUI**: Project "main page" that shows all the metadata
- [ ] **GUI**: NCER/NANR editors
- [ ] **GUI**: Undo/Redo
    - [ ] Show an asterisk on tabs with unsaved changes
- [ ] **GUI**: Warn when closing without saving
- [ ] **GUI**: Open recent
- [ ] **GUI**: Import/export portable project
- [ ] **GUI**: Settings
    - [ ] Light/dark mode saving
    - [ ] Recent project list

### Versions 1.1+
- [ ] **Core**: BNCAD format
- [ ] **Core**: Support alternative files that use Nintendo formats (Layton 1 comes to mind)
- [ ] **fission**: Polish/improve
- [ ] **GUI**: Import/export BNCAD
- [ ] **GUI**: Tile-by-tile editing for NSCR and NCGR


## Credits
* Contributors: me! (patataofcourse)
* Documentation on file formats:
    - [This](https://www.romhacking.net/documents/%5b469%5dnds_formats.htm) document in romhacking.net
    - [Tinke](https://www.github.com/pleonex/tinke) source code
    - [gbatek](https://problemkaputt.de/gbatek.htm)
* ThePurpleAnon for the very cool name idea