# Version 0.7.0 (2022-08-22)

## New formats support

- Android Binary XML - `xml`
- Android Compiled Resources - `arsc`
- Optimized Dalvik Executable - `dey`

# Version 0.6.0 (2021-12-18)

## Improvements

- Add file format detection from bytes

# Version 0.5.0 (2021-12-12)

## API

- Drop ZIP-based file formats support
- Switch back FileFormat type from a structure to an enum

## Improvements

- Add precision to the `Dalvik Executable` signature
- Switch to Rust 2021

## New formats support

- Java KeyStore - `jks`

# Version 0.4.0 (2021-10-22)

## Docs

- Reorganize supported file formats table

## Improvements

- Add tests for all `High Efficiency Image Coding Sequence` format
- Add tests for all `High Efficiency Image Coding` format

## New formats support

- 3D Manufacturing Format - `3mf`
- Java Archive - `jar`
- Microsoft DirectDraw Surface - `dds`
- Microsoft Visio Drawing - `vsdx`
- Office Open XML Document - `docx`
- Office Open XML Presentation - `pptx`
- Office Open XML Workbook - `xlsx`
- Radiance HDR - `hdr`
- Web Application Resource - `war`
- XAP - `xap`
- XPInstall - `xpi`

# Version 0.3.0 (2021-10-18)

## API

- Switch FileFormat type from an enum to a structure

## Fixes

- Use of the correct `Tag Image File Format` extension

## Improvements

- Add new `Apple QuickTime` signatures
- Add new `Audio Interchange File Format` signature
- Add precision to the `Debian Binary Package` signature
- Add precision to the `Flexible Image Transport System` signature
- Add precision to the `Windows Media Video` signature
- Add precision to the `Windows Shortcut` signature
- Improve support of some formats
- Replace `Windows Installer` by `Compound File Binary`

## New formats support

- ALZ - `alz`
- ANI - `ani`
- Adobe Flash Player Audio - `f4a`
- Adobe Flash Player Audiobook - `f4b`
- Apache Arrow Columnar - `arrow`
- Apple iTunes Audiobook - `m4b`
- CUR - `cur`
- Canon Raw 2 - `cr2`
- Canon Raw 3 - `cr3`
- FastTracker 2 Extended Module - `xm`
- Fujifilm Raw - `raf`
- Impulse Tracker Module - `it`
- LHA - `lzh`
- Lempel–Ziv Finite State Entropy - `lzfse`
- Microsoft Compiled HTML Help - `chm`
- Microsoft Virtual Hard Disk - `vhd`
- Microsoft Virtual Hard Disk 2 - `vhdx`
- Nikon Electronic File - `nef`
- Panasonic Raw - `rw2`
- Qualcomm PureVoice - `qcp`
- ScreamTracker 3 Module - `s3m`
- SeqBox - `sbx`
- Snappy - `sz`
- Sony DSD Stream File - `dsf`
- cpio - `cpio`
- macOS Alias - `alias`
- zoo - `zoo`

# Version 0.2.1 (2021-10-14)

## Fixes

- Fix `Tag Image File Format` signature

# Version 0.2.0 (2021-10-07)

## New formats support

- Animated Portable Network Graphics - `apng`
- BDAV MPEG-2 Transport Stream - `m2ts`
- Electronic Publication - `epub`
- Game Boy Color ROM - `gbc`
- HyperText Markup Language - `html`
- Khronos Texture - `ktx`
- Khronos Texture 2 - `ktx2`
- Material Exchange Format - `mxf`
- Mobipocket - `mobi`
- Olympus Raw Format - `orf`
- OpenDocument Graphics - `odg`
- OpenDocument Presentation - `odp`
- OpenDocument Spreadsheet - `ods`
- OpenDocument Text - `odt`
- Rich Text Format - `rtf`
- Shapefile - `shp`
- SketchUp - `skp`
- UNIX archiver - `ar`

# Version 0.1.0 (2021-10-03)

First version.
