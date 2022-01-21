# nuclear
Python library and CLI for Nintendo DS image formats, based off ndspy.

## Requirements
* Python **3.10 or higher**
* ndspy (not used yet but will be necessary in the future)

## How to use
1. Make a folder named `test_files`
2. Put a NCLR file called backbeat.NCLR inside
3. `python main.py`
4. Profit

## Planned stuff
At some point, nuclear should be able to offer an easy way to deal with Nintendo DS cell (sprite) files and animations, in a way somewhat similar to BRCAD/BCCAD's [Bread](https://www.github.com/rhmodding/bread).

Also, tools to open and manage/fuck with the files will be left easily available in the form of the nuclear library, as well as an easy-to-use CLI.

## Progress
Currently this isn't very advanced, the only thing it can do is open palette files and it still breaks lmao

## Credits
* Contributors: me! (patataofcourse)
* Documentation on file formats:
    - [This](https://www.romhacking.net/documents/%255b469%255dnds_formats.htm) document in romhacking.net
    - [Tinke](https://www.github.com/pleonex/tinke) source code
* RoadrunnerWMC for ndspy
* ThePurpleAnon for the very cool name idea