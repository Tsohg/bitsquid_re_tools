# bitsquid_re_tools
Reverse engineering toolchain for the bitsquid engine.
Tools Roadmap:
- [X] Asset Unbundler
- [ ] Luajit Decompiler
- [ ] .timpani_bank Extractor
- [ ] Luajit Disassembly Editor
- [ ] Lua State Hijacking (Allowing your own lua code to load and run when the game does).
- [ ] Murmur32 Rainbow Table (Create a rainbow table by detouring the bitsquid engine's hashing function).
- [ ] Directory and File Name Restorer (Using the rainbow table, reconstruct the file structure).
- [ ] .dds to .png Conversion

# Usage

```
bitsquid_re_tools.exe -t TOOL_NAME [OPTIONS]
where TOOL_NAME is the name of a supported tool in the toolchain.

-t --tool <TOOL> Currently supported tools: bitsquid_unbundler
-i --input <INPUT> Input may be a path to a file or directory. A default input may be substituted depending on the tool used.
-o --output <OUTPUT> Output may be a path to a file or a directory. A default output may be substituted depending on the tool used. (Typically, the pwd).
```
