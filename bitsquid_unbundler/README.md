# Build
Clone the repo.
Open a command prompt in the repo directory. If you see the README you are in the right place.
```
cargo build --release
```
The executable should now be in the /target/ directory.

# Usage
```
Usage: bitsquid_unbundler.exe [OPTIONS]

Options:
  -i, --indir <INPUT_DIRECTORY>    [Optional] The input directory containing the bitsquid compiled assets.
                                        Defaults to searching the Steam directory for Magicka: Wizard Wars' data_win32_bundled directory.
  -o, --outdir <OUTPUT_DIRECTORY>  [Optional] The output directory which the extracted files shall be written to.
                                        Defaults to creating a directory in the present working directory of the executable for output.
  -h, --help                       Print help
  -V, --version                    Print version           Print version
```

# File Format of Bundles
The file format of a bundle is a collection of zlib files. The first step is to uncompress the zlib files into its own file. The resulting file contains all the files which need to be split off into their own file. (Separated into Bundle Unpacking then File Unpacking).

## Bundle Unpacking
| size  | contents |
| ------------- | ------------- |
| u32  | file signature (0xf0000004)  |
| u32 | sum of all file sizes when uncompressed |
| u32 | reserved space |
| repeat until no more bytes | -- |
| u32 | zlib block length |
| zlib block length | zlib block. If the block length equals (1<<16) the block is already uncompressed. Otherwise decompress the block. |

## File Unpacking
| size  | contents |
| ------------- | ------------- |
| u32 | (file count) the number of files that exist within the file |
| 256 bytes | checksum of some kind. |
| 16 * (file count) | file names and extensions (repeated below for some reason) |
| repeat for (file count) iterations | -- |
| u64 | murmur32 hashed file extension. see file_writer.rs for the lookup table. |
| u64 | murmur32 hashed file path |
| u64 | flag for if there is data to be read from the stream. |
| assuming there is data | -- |
| u32 | some kind of flag |
| u32 | (file size) the size of the file |
| u32 | unknown at present |
| (file size) | the file's data. this is what can be split off into its own file with the path as its name and the file extension after the string is looked up. |

### Side Note
This unbundler ignores .stream, .data, and .ini files in the bundled directory if they exist.
