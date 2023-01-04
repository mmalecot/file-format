# Version 0.13.0 (unreleased)

## API

- Add `FileFormat::display_name`
- Add `FileFormat::short_name`
- Replace `FileFormat::name` with `FileFormat::display_name` in `impl Display for FileFormat`

## Fixes

- Fix LHA signatures

## New formats support

- LArc
- PMarc

# Version 0.12.0 (2022-12-30)

## API

- Add `accuracy` feature

## Docs

- Improves global documentation
- Simplify `lefi` example

## New formats support

- LaTeX

# Version 0.11.0 (2022-12-21)

## Docs

- Remove dependencies of `lefi` example

## New formats support

- Clojure Script
- Extensible Markup Language
- Extensible Stylesheet Language Transformations
- Geography Markup Language
- HyperText Markup Language
- Keyhole Markup Language
- Lua Script
- MusicXML
- MusicXML Zipped
- Perl Script
- Plain Text
- Python Script
- Really Simple Syndication
- Ruby Script
- Scalable Vector Graphics
- Shell Script
- Simple Object Access Protocol
- Tool Command Language Script
- vCalendar
- vCard

# Version 0.10.0 (2022-12-04)

## Fixes

- Fix inverted names between `MatroskaVideo` and `MetaInformationEncapsulation`

## New formats support

- Common Object File Format
- Google Draco
- ICC Profile
- Mach-O
- OpenRaster

# Version 0.9.1 (2022-11-30)

## Docs

- Fix `README.md`

# Version 0.9.0 (2022-11-30)

## API

- Add `FileFormat::kind`
- Rename `AdobeInDesignDocument` to `AdobeIndesignDocument`
- Rename `Ani` to `WindowsAnimatedCursor`
- Rename `AppleQuickTime` to `AppleQuicktime`
- Rename `Cur` to `WindowsCursor`
- Rename `EmbeddedOpenType` to `EmbeddedOpentype`
- Rename `Ico` to `WindowsIcon`
- Rename `JavaKeyStore` to `JavaKeystore`
- Rename `MacOsAlias` to `MacosAlias`
- Rename `MicrosoftVisioDrawing` to `OfficeOpenXmlDrawing`
- Rename `MpegAudioLayer3` to `Mpeg12AudioLayer3`
- Rename `OfficeOpenXmlWorkbook` to `OfficeOpenXmlSpreadsheet`
- Rename `OpenExr` to `Openexr`
- Rename `OpenType` to `Opentype`
- Rename `ScreamTracker3Module` to `Screamtracker3Module`
- Rename `SeqBox` to `Seqbox`
- Rename `SketchUp` to `Sketchup`
- Rename `TrueType` to `Truetype`
- Rename `VirtualBoxVirtualDiskImage` to `VirtualboxVirtualDiskImage`
- Rename `WavPack` to `Wavpack`
- Rename `WebAssemblyBinary` to `WebassemblyBinary`
- Rename `WebP` to `Webp`
- Rename `WindowsExecutable` to `MsDosExecutable`
- Rename `XpInstall` to `Xpinstall`

## Fixes

- Add MPEG-1/2 Audio Layer 3 signature
- Add Matroska Video signature
- Change UNIX archiver extension from `ar` to `a` (preferred)
- Fix Apple QuickTime signature
- Fix Joint Photographic Experts Group signature

## Internal changes

- Add `formats` macro
- Split items into modules

## New formats support

- Adobe Illustrator Artwork
- Circuit Diagram Document
- Creative Voice
- DER Certificate
- DjVu
- Dynamic Link Library
- Encapsulated PostScript
- Enterprise Application Archive
- Keyhole Markup Language Zipped
- LLVM Bitcode
- Lua Bytecode
- MPEG-1 Audio Layer 1
- MPEG-1 Audio Layer 2
- MPEG-2 Transport Stream
- Meta Information Encapsulation
- Microsoft Access 2007 Database
- Microsoft Access Database
- Microsoft Excel Spreadsheet
- Microsoft PowerPoint Presentation
- Microsoft Project Plan
- Microsoft Publisher Document
- Microsoft Software Installer
- Microsoft Visio Drawing
- Microsoft Word Document
- PEM Certificate
- PEM Certificate Signing Request
- PEM Private Key
- PGP Message
- PGP Private Key Block
- PGP Public Key Block
- PGP Signature
- PGP Signed Message
- Portable Executable
- PostScript
- Rich Text Format
- Sony Movie
- TASTy
- Web Application Archive
- WebM
- Windows App Package
- iOS App Store Package

# Version 0.8.0 (2022-11-06)

## API

- Add `FileFormat::from_reader`
- Add `impl From<&[u8]> for FileFormat`

## Docs

- Add `lefi` example
- Add `CHANGELOG.md`

## Internal changes

- Make signature offset optional
- Remove FileFormat enum generation with macro
- Simplify `signatures` macro

## New formats support

- 3D Manufacturing Format
- Android Package
- Design Web Format XPS
- Electronic Publication
- Java Archive
- Microsoft Visio Drawing
- Microsoft Visual Studio Extension
- Office Open XML Document
- Office Open XML Presentation
- Office Open XML Workbook
- OpenDocument Graphics
- OpenDocument Presentation
- OpenDocument Spreadsheet
- OpenDocument Text
- XAP
- XPInstall

# Version 0.7.0 (2022-08-22)

## New formats support

- Android Binary XML
- Android Compiled Resources
- Optimized Dalvik Executable

# Version 0.6.0 (2021-12-18)

## API

- Add `FileFormat::from_bytes`

# Version 0.5.0 (2021-12-12)

## API

- Switch back FileFormat type from a structure to an enum

## Discontinued formats

- 3D Manufacturing Format
- BDAV MPEG-2 Transport Stream
- Java Archive
- Microsoft Visio Drawing
- Office Open XML Document
- Office Open XML Presentation
- Office Open XML Workbook
- OpenDocument Graphics
- OpenDocument Presentation
- OpenDocument Spreadsheet
- OpenDocument Text
- Web Application Resource
- XAP
- XPInstall

## Improvements

- Add precision to the Dalvik Executable signature
- Switch to Rust 2021

## New formats support

- Java KeyStore

# Version 0.4.0 (2021-10-22)

## Docs

- Reorganize supported file formats table

## Improvements

- Add tests for all High Efficiency Image Coding Sequence format
- Add tests for all High Efficiency Image Coding format

## New formats support

- 3D Manufacturing Format
- Java Archive
- Microsoft DirectDraw Surface
- Microsoft Visio Drawing
- Office Open XML Document
- Office Open XML Presentation
- Office Open XML Workbook
- Radiance HDR
- Web Application Resource
- XAP
- XPInstall

# Version 0.3.0 (2021-10-18)

## API

- Switch FileFormat type from an enum to a structure

## Discontinued formats

- HyperText Markup Language

## Fixes

- Use of the correct Tag Image File Format extension

## Improvements

- Add new Apple QuickTime signatures
- Add new Audio Interchange File Format signature
- Add precision to the Debian Binary Package signature
- Add precision to the Flexible Image Transport System signature
- Add precision to the Windows Media Video signature
- Add precision to the Windows Shortcut signature
- Improve support of some formats
- Replace Windows Installer by Compound File Binary

## New formats support

- ALZ
- ANI
- Adobe Flash Player Audio
- Adobe Flash Player Audiobook
- Apache Arrow Columnar
- Apple iTunes Audiobook
- CUR
- Canon Raw 2
- Canon Raw 3
- FastTracker 2 Extended Module
- Fujifilm Raw
- Impulse Tracker Module
- LHA
- Lempel–Ziv Finite State Entropy
- Microsoft Compiled HTML Help
- Microsoft Virtual Hard Disk
- Microsoft Virtual Hard Disk 2
- Nikon Electronic File
- Panasonic Raw
- Qualcomm PureVoice
- ScreamTracker 3 Module
- SeqBox
- Snappy
- Sony DSD Stream File
- cpio
- macOS Alias
- zoo

# Version 0.2.1 (2021-10-14)

## Fixes

- Fix Tag Image File Format signature

# Version 0.2.0 (2021-10-07)

## New formats support

- Animated Portable Network Graphics
- BDAV MPEG-2 Transport Stream
- Electronic Publication
- Game Boy Color ROM
- HyperText Markup Language
- Khronos Texture
- Khronos Texture 2
- Material Exchange Format
- Mobipocket
- Olympus Raw Format
- OpenDocument Graphics
- OpenDocument Presentation
- OpenDocument Spreadsheet
- OpenDocument Text
- Rich Text Format
- Shapefile
- SketchUp
- UNIX archiver

# Version 0.1.0 (2021-10-03)

First version.
