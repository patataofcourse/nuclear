# nuclear
Rust library and tool for opening various graphics-related Nintendo DS files

## Roadmap

### Version 0.1 (#8CF)
- [x] **Core**: Support NCLR, NCGR, NSCR
- [x] **Core**: Project format with wrappers, allow loading and saving it
- [ ] **GUI**: Load/create/save projects
    - missing saving
- [x] **GUI**: Edit project metadata
- [ ] **GUI**: Editors for the implemented formats
    - [x] NCLR
    - [ ] NCGR
        - in progress
    - [ ] NSCR
- [ ] **GUI**: Import Nintendo files into project
- [ ] **GUI**: Open files with sidebar
    - in progress

### Versions 0.2 (#F88) - 1.0 (#20F)
- [ ] **Core:** Import PNG into NSCR (smart/GRIT-like conversion)
- [ ] **Core**: NCER / NANR support
- [ ] **Core**: Portable project format
- [ ] **fission**: Get a basic version of the framework
    - this objective will be detailed more in the future
- [ ] **GUI**: Project "main page" that shows all the metadata
- [ ] **GUI**: NCER/NANR editors
- [ ] **GUI**: Undo/Redo
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

## Credits
* Contributors: me! (patataofcourse)
* Documentation on file formats:
    - [This](https://www.romhacking.net/documents/%5b469%5dnds_formats.htm) document in romhacking.net
    - [Tinke](https://www.github.com/pleonex/tinke) source code
    - [gbatek](https://problemkaputt.de/gbatek.htm)
* ThePurpleAnon for the very cool name idea