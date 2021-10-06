#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[macro_use]
mod macros;

file_format! {
  - format: Odp
    description: "OpenDocument Presentation (ODP)"
    media_type: "application/vnd.oasis.opendocument.presentation"
    extensions: ["odp", "fodp"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x50\x4B\x03\x04"
        - offset: 30
          value: b"mimetype"
        - offset: 38
          value: b"application/vnd.oasis.opendocument.presentation"

  - format: Ods
    description: "OpenDocument SpreadSheet (ODS)"
    media_type: "application/vnd.oasis.opendocument.spreadsheet"
    extensions: ["ods", "fods"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x50\x4B\x03\x04"
        - offset: 30
          value: b"mimetype"
        - offset: 38
          value: b"application/vnd.oasis.opendocument.spreadsheet"

  - format: Odg
    description: "OpenDocument Graphics (ODG)"
    media_type: "application/vnd.oasis.opendocument.graphics"
    extensions: ["odg", "fodg"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x50\x4B\x03\x04"
        - offset: 30
          value: b"mimetype"
        - offset: 38
          value: b"application/vnd.oasis.opendocument.graphics"

  - format: Odt
    description: "OpenDocument Text (ODT)"
    media_type: "application/vnd.oasis.opendocument.text"
    extensions: ["odt", "fodt"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x50\x4B\x03\x04"
        - offset: 30
          value: b"mimetype"
        - offset: 38
          value: b"application/vnd.oasis.opendocument.text"

  - format: Vdi
    description: "VirtualBox Virtual Disk Image (VDI)"
    media_type: "application/x-virtualbox-vdi"
    extensions: ["vdi"]
    signatures:
      - parts:
        - offset: 0
          value: b"<<< Oracle VM VirtualBox Disk Image >>>"

  - format: Epub
    description: "Electronic Publication (EPUB)"
    media_type: "application/epub+zip"
    extensions: ["epub"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x50\x4B\x03\x04"
        - offset: 30
          value: b"mimetype"
        - offset: 38
          value: b"application/epub+zip"

  - format: Skp
    description: "SketchUp Document"
    media_type: "application/vnd.sketchup.skp"
    extensions: ["skp"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xFF\xFE\xFF\x0E\x53\x00\x6B\x00\x65\x00\x74\x00\x63\x00\x68\x00"
        - offset: 16
          value: b"\x55\x00\x70\x00\x20\x00\x4D\x00\x6F\x00\x64\x00\x65\x00\x6C\x00"

  - format: Deb
    description: "Debian package"
    media_type: "application/x-deb"
    extensions: ["deb"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x21\x3C\x61\x72\x63\x68\x3E\x0A"
        - offset: 8
          value: b"debian-binary"

  - format: InDesignDocument
    description: "Adobe InDesign document"
    media_type: "application/x-indesign"
    extensions: ["indd"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x06\x06\xED\xF5\xD8\x1D\x46\xE5\xBD\x31\xEF\xE7\xFE\x74\xB7\x1D"

  - format: Sqlite3
    description: "SQLite 3 Database"
    media_type: "application/vnd.sqlite3"
    extensions: ["sqlite", "sqlite3", "sqlitedb", "db"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x53\x51\x4C\x69\x74\x65\x20\x66\x6F\x72\x6D\x61\x74\x20\x33\x00"

  - format: Html
    description: "HyperText Markup Language (HTML)"
    media_type: "text/html"
    extensions: ["html", "htm"]
    signatures:
      - parts:
        - offset: 0
          value: b"<!DOCTYPE HTML>"
      - parts:
        - offset: 0
          value: b"<!DOCTYPE html>"
      - parts:
        - offset: 0
          value: b"<!doctype html>"

  - format: ICalendar
    description: "iCalendar"
    media_type: "text/calendar"
    extensions: ["ics", "ical", "ifb", "icalendar"]
    signatures:
      - parts:
        - offset: 0
          value: b"BEGIN:VCALENDAR"

  - format: Apng
    description: "Animated Portable Network Graphics (APNG)"
    media_type: "image/apng"
    extensions: ["apng"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"
        - offset: 0x25
          value: b"acTL"

  - format: Jpeg
    description: "Joint Photographic Experts Group (JPEG)"
    media_type: "image/jpeg"
    extensions: ["jpg", "jpeg"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xFF\xD8\xFF\xE0\x00\x10\x4A\x46\x49\x46\x00\x01"
      - parts:
        - offset: 0
          value: b"\xFF\xD8\xFF\xE1"
        - offset: 6
          value: b"\x45\x78\x69\x66\x00\x00"
      - parts:
        - offset: 0
          value: b"\xFF\xD8\xFF\xDB"
      - parts:
        - offset: 0
          value: b"\xFF\xD8\xFF\xEE"

  - format: JpegXl
    description: "JPEG XL"
    media_type: "image/jxl"
    extensions: ["jxl"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x00\x00\x00\x0C\x4A\x58\x4C\x20\x0D\x0A\x87\x0A"
      - parts:
        - offset: 0
          value: b"\xFF\x0A"

  - format: Ktx
    description: "Khronos TeXture (KTX)"
    media_type: "image/ktx"
    extensions: ["ktx"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xAB\x4B\x54\x58\x20\x31\x31\xBB\x0D\x0A\x1A\x0A"

  - format: Ktx2
    description: "Khronos TeXture 2 (KTX2)"
    media_type: "image/ktx2"
    extensions: ["ktx2"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xAB\x4B\x54\x58\x20\x32\x30\xBB\x0D\x0A\x1A\x0A"

  - format: Matroska
    description: "Matroska Multimedia Container"
    media_type: "video/x-matroska"
    extensions: ["mkv"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x1A\x45\xDF\xA3"
        - offset: 24
          value: b"matroska"

  - format: VCard
    description: "vCard"
    media_type: "text/vcard"
    extensions: ["vcf", "vcard"]
    signatures:
      - parts:
        - offset: 0
          value: b"BEGIN:VCARD"

  - format: Fits
    description: "Flexible Image Transport System (FITS)"
    media_type: "image/fits"
    extensions: ["fits", "fit", "fts"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x53\x49\x4D\x50\x4C\x45\x20\x20\x3D\x20"

  - format: OggAudio
    description: "Ogg audio"
    media_type: "audio/ogg"
    extensions: ["ogg", "oga", "spx"]
    signatures:
      - parts:
        - offset: 0
          value: b"OggS"
        - offset: 29
          value: b"vorbis"
      - parts:
        - offset: 0
          value: b"OggS"
        - offset: 28
          value: b"Speex"
      - parts:
        - offset: 0
          value: b"OggS"
        - offset: 29
          value: b"FLAC"

  - format: OggVideo
    description: "Ogg video"
    media_type: "video/ogg"
    extensions: ["ogv"]
    signatures:
      - parts:
        - offset: 0
          value: b"OggS"
        - offset: 29
          value: b"theora"

  - format: QuickTime
    description: "QuickTime Movie"
    media_type: "video/quicktime"
    extensions: ["mov", "qt"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x00\x00\x00\x14"
        - offset: 4
          value: b"ftypqt"

  - format: Wmv
    description: "Windows Media Video (WMV)"
    media_type: "video/x-ms-asf"
    extensions: ["wmv", "wm", "asf"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x30\x26\xB2\x75\x8E\x66\xCF\x11\xA6\xD9"

  - format: GbcRom
    description: "Game Boy Color ROM"
    media_type: "application/x-gameboy-color-rom"
    extensions: ["gbc"]
    signatures:
      - parts:
        - offset: 0x104
          value: b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B"
        - offset: 0x143
          value: b"\x80"
      - parts:
        - offset: 0x104
          value: b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B"
        - offset: 0x143
          value: b"\xC0"

  - format: Lzop
    description: "Lzop"
    media_type: "application/x-lzop"
    extensions: ["lzo"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x89\x4C\x5A\x4F\x00\x0D\x0A\x1A\x0A"

  - format: Orf
    description: "Olympus Raw Format (ORF)"
    media_type: "image/x-olympus-orf"
    extensions: ["orf"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x49\x49\x52\x4F\x08\x00\x00\x00\x18"

  - format: Aiff
    description: "Audio Interchange File Format (AIFF)"
    media_type: "audio/aiff"
    extensions: ["aif", "aiff"]
    signatures:
      - parts:
        - offset: 0
          value: b"FORM"
        - offset: 8
          value: b"AIFF"

  - format: Mobi
    description: "Mobipocket"
    media_type: "application/x-mobipocket-ebook"
    extensions: ["mobi"]
    signatures:
      - parts:
        - offset: 60
          value: b"BOOKMOBI"

  - format: Avi
    description: "Audio Video Interleave (AVI)"
    media_type: "video/avi"
    extensions: ["avi"]
    signatures:
      - parts:
        - offset: 0
          value: b"RIFF"
        - offset: 8
          value: b"\x41\x56\x49\x20"

  - format: Avif
    description: "AV1 Image File Format (AVIF)"
    media_type: "image/avif"
    extensions: ["avif"]
    signatures:
      - parts:
        - offset: 4
          value: b"ftypavif"

  - format: GbRom
    description: "Game Boy ROM"
    media_type: "application/x-gameboy-rom"
    extensions: ["gb"]
    signatures:
      - parts:
        - offset: 0x104
          value: b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B"

  - format: GbaRom
    description: "Game Boy Advance ROM"
    media_type: "application/x-gba-rom"
    extensions: ["gba"]
    signatures:
      - parts:
        - offset: 4
          value: b"\x24\xFF\xAE\x51\x69\x9A\xA2\x21"

  - format: Heif
    description: "High Efficiency Image File Format (HEIF)"
    media_type: "image/heic"
    extensions: ["heic"]
    signatures:
      - parts:
        - offset: 4
          value: b"ftypheic"
      - parts:
        - offset: 4
          value: b"ftypheix"

  - format: Lnk
    description: "Windows shortcut"
    media_type: "application/x-ms-shortcut"
    extensions: ["lnk", "url", "cda"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x4C\x00\x00\x00\x01\x14\x02\x00"

  - format: Mp4
    description: "MPEG-4 Part 14 (MP4)"
    media_type: "video/mp4"
    extensions: ["mp4"]
    signatures:
      - parts:
        - offset: 4
          value: b"ftypavc1"
      - parts:
        - offset: 4
          value: b"ftypdash"
      - parts:
        - offset: 4
          value: b"ftypiso2"
      - parts:
        - offset: 4
          value: b"ftypiso3"
      - parts:
        - offset: 4
          value: b"ftypiso4"
      - parts:
        - offset: 4
          value: b"ftypiso5"
      - parts:
        - offset: 4
          value: b"ftypiso6"
      - parts:
        - offset: 4
          value: b"ftypisom"
      - parts:
        - offset: 4
          value: b"ftypmmp4"
      - parts:
        - offset: 4
          value: b"ftypmp41"
      - parts:
        - offset: 4
          value: b"ftypmp42"
      - parts:
        - offset: 4
          value: b"ftypmp4v"
      - parts:
        - offset: 4
          value: b"ftypmp71"
      - parts:
        - offset: 4
          value: b"ftypMSNV"
      - parts:
        - offset: 4
          value: b"ftypNDAS"
      - parts:
        - offset: 4
          value: b"ftypNDSC"
      - parts:
        - offset: 4
          value: b"ftypNDSH"
      - parts:
        - offset: 4
          value: b"ftypNDSM"
      - parts:
        - offset: 4
          value: b"ftypNDSP"
      - parts:
        - offset: 4
          value: b"ftypNDSS"
      - parts:
        - offset: 4
          value: b"ftypNDXC"
      - parts:
        - offset: 4
          value: b"ftypNDXH"
      - parts:
        - offset: 4
          value: b"ftypNDXM"
      - parts:
        - offset: 4
          value: b"ftypNDXP"
      - parts:
        - offset: 4
          value: b"ftypF4V"
      - parts:
        - offset: 4
          value: b"ftypF4P"

  - format: Msi
    description: "Windows Installer"
    media_type: "application/x-ole-storage"
    extensions: ["msi", "msp"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1"

  - format: N64Rom
    description: "Nintendo 64 ROM"
    media_type: "application/x-n64-rom"
    extensions: ["z64"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x80\x37\x12\x40\x00\x00\x00\x0F"
      - parts:
        - offset: 0
          value: b"\x37\x80\x40\x12\x00\x00\x0F\x00"
      - parts:
        - offset: 0
          value: b"\x12\x40\x80\x37\x00\x0F\x00\x00"
      - parts:
        - offset: 0
          value: b"\x40\x12\x37\x80\x0F\x00\x00\x00"

  - format: NdsRom
    description: "Nintendo DS ROM"
    media_type: "application/x-nintendo-ds-rom"
    extensions: ["nds"]
    signatures:
      - parts:
        - offset: 0xC0
          value: b"\x24\xFF\xAE\x51\x69\x9A\xA2\x21"
      - parts:
        - offset: 0xC0
          value: b"\xC8\x60\x4F\xE2\x01\x70\x8F\xE2"

  - format: Png
    description: "Portable Network Graphics (PNG)"
    media_type: "image/png"
    extensions: ["png"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"

  - format: Rar
    description: "Roshal ARchive (RAR)"
    media_type: "application/vnd.rar"
    extensions: ["rar"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x52\x61\x72\x21\x1A\x07\x01\x00"
      - parts:
        - offset: 0
          value: b"\x52\x61\x72\x21\x1A\x07\x00"

  - format: Tar
    description: "Tape archive (tar)"
    media_type: "application/x-tar"
    extensions: ["tar"]
    signatures:
      - parts:
        - offset: 257
          value: b"\x75\x73\x74\x61\x72\x00\x30\x30"
      - parts:
        - offset: 257
          value: b"\x75\x73\x74\x61\x72\x20\x20\x00"

  - format: Wave
    description: "Waveform Audio (WAVE)"
    media_type: "audio/vnd.wave"
    extensions: ["wav"]
    signatures:
      - parts:
        - offset: 0
          value: b"RIFF"
        - offset: 8
          value: b"WAVE"

  - format: WebM
    description: "WebM"
    media_type: "video/webm"
    extensions: ["webm"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x1A\x45\xDF\xA3"
        - offset: 24
          value: b"webm"

  - format: WebP
    description: "WebP"
    media_type: "image/webp"
    extensions: ["webp"]
    signatures:
      - parts:
        - offset: 0
          value: b"RIFF"
        - offset: 8
          value: b"WEBP"

  - format: Xcf
    description: "eXperimental Computing Facility (XCF)"
    media_type: "image/x-xcf"
    extensions: ["xcf"]
    signatures:
      - parts:
        - offset: 0
          value: b"gimp xcf"

  - format: Ar
    description: "Archiver"
    media_type: "application/x-archive"
    extensions: ["ar", "a", "lib"]
    signatures:
      - parts:
        - offset: 0
          value: b"!<arch>"

  - format: Blender
    description: "Blender 3D Data"
    media_type: "application/x-blender"
    extensions: ["blend"]
    signatures:
      - parts:
        - offset: 0
          value: b"BLENDER"

  - format: Jp2
    description: "JPEG 2000"
    media_type: "image/jp2"
    extensions: ["jp2"]
    signatures:
      - parts:
        - offset: 16
          value: b"ftypjp2"

  - format: M4a
    description: "Audio-only MPEG-4"
    media_type: "audio/x-m4a"
    extensions: ["m4a"]
    signatures:
      - parts:
        - offset: 4
          value: b"ftypM4A"

  - format: M4v
    description: "M4V"
    media_type: "video/x-m4v"
    extensions: ["m4v"]
    signatures:
      - parts:
        - offset: 4
          value: b"ftypM4V"

  - format: ThirdGpp
    description: "3rd Generation Partnership Project (3GPP)"
    media_type: "video/3gpp"
    extensions: ["3gp"]
    signatures:
      - parts:
        - offset: 4
          value: b"ftyp3gp"

  - format: ThirdGpp2
    description: "3rd Generation Partnership Project 2 (3GPP2)"
    media_type: "video/3gpp2"
    extensions: ["3g2"]
    signatures:
      - parts:
        - offset: 4
          value: b"ftyp3g2"

  - format: Gif
    description: "Graphics Interchange Format (GIF)"
    media_type: "image/gif"
    extensions: ["gif"]
    signatures:
      - parts:
        - offset: 0
          value: b"GIF87a"
      - parts:
        - offset: 0
          value: b"GIF89a"

  - format: SevenZip
    description: "7-Zip"
    media_type: "application/x-7z-compressed"
    extensions: ["7z"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x37\x7A\xBC\xAF\x27\x1C"

  - format: Xml
    description: "Extensible Markup Language (XML)"
    media_type: "text/xml"
    extensions: ["xml"]
    signatures:
      - parts:
        - offset: 0
          value: b"<?xml "

  - format: Xz
    description: "XZ"
    media_type: "application/x-xz"
    extensions: ["xz"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xFD\x37\x7A\x58\x5A\x00"

  - format: Amr
    description: "Adaptive Multi-Rate (AMR)"
    media_type: "audio/amr"
    extensions: ["amr", "3ga"]
    signatures:
      - parts:
        - offset: 0
          value: b"#!AMR"

  - format: Eot
    description: "Embedded OpenType (EOT)"
    media_type: "application/vnd.ms-fontobject"
    extensions: ["eot"]
    signatures:
      - parts:
        - offset: 8
          value: b"\x00\x00\x01"
        - offset: 34
          value: b"\x4C\x50"
      - parts:
        - offset: 8
          value: b"\x01\x00\x02"
        - offset: 34
          value: b"\x4C\x50"
      - parts:
        - offset: 8
          value: b"\x02\x00\x02"
        - offset: 34
          value: b"\x4C\x50"

  - format: Iso
    description: "ISO image"
    media_type: "application/x-iso9660-image"
    extensions: ["iso"]
    signatures:
      - parts:
        - offset: 0x8001
          value: b"CD001"
      - parts:
        - offset: 0x8801
          value: b"CD001"
      - parts:
        - offset: 0x9001
          value: b"CD001"

  - format: OpenType
    description: "OpenType"
    media_type: "font/otf"
    extensions: ["otf"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x4F\x54\x54\x4F\x00"

  - format: Pdf
    description: "Portable Document Format (PDF)"
    media_type: "application/pdf"
    extensions: ["pdf"]
    signatures:
      - parts:
        - offset: 0
          value: b"%PDF-"

  - format: Rtf
    description: "Rich Text Format (RTF)"
    media_type: "text/rtf"
    extensions: ["rtf"]
    signatures:
      - parts:
        - offset: 0
          value: b"{\\rtf"

  - format: TrueType
    description: "TrueType"
    media_type: "font/ttf"
    extensions: ["ttf"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x00\x01\x00\x00\x00"

  - format: Ape
    description: "Monkey's Audio"
    media_type: "audio/x-ape"
    extensions: ["ape"]
    signatures:
      - parts:
        - offset: 0
          value: b"MAC "

  - format: AppleIconImage
    description: "Apple Icon Image"
    media_type: "image/icns"
    extensions: ["icns"]
    signatures:
      - parts:
        - offset: 0
          value: b"icns"

  - format: Bpg
    description: "Better Portable Graphics (BPG)"
    media_type: "image/bpg"
    extensions: ["bpg"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x42\x50\x47\xFB"

  - format: Cab
    description: "Cabinet (CAB)"
    media_type: "application/vnd.ms-cab-compressed"
    extensions: ["cab"]
    signatures:
      - parts:
        - offset: 0
          value: b"MSCF"
      - parts:
        - offset: 0
          value: b"ISc("

  - format: ChromeExtension
    description: "Google Chrome Extension"
    media_type: "application/x-google-chrome-extension"
    extensions: ["crx"]
    signatures:
      - parts:
        - offset: 0
          value: b"Cr24"

  - format: Cineon
    description: "Cineon Image"
    media_type: "image/cineon"
    extensions: ["cin"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x80\x2A\x5F\xD7"

  - format: Dex
    description: "Dalvik Executable"
    media_type: "application/vnd.android.dex"
    extensions: ["dex"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x64\x65\x78\x0a"

  - format: Dicom
    description: "Digital Imaging and Communications in Medicine (DICOM)"
    media_type: "application/dicom"
    extensions: ["dcm"]
    signatures:
      - parts:
        - offset: 128
          value: b"\x44\x49\x43\x4D"

  - format: Dpx
    description: "Digital Picture Exchange (DPX)"
    media_type: "image/x-dpx"
    extensions: ["dpx"]
    signatures:
      - parts:
        - offset: 0
          value: b"SDPX"
      - parts:
        - offset: 0
          value: b"XPDS"

  - format: Elf
    description: "Executable and Linkable Format (ELF)"
    media_type: "application/x-executable"
    extensions: ["elf", "so"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x7F\x45\x4C\x46"

  - format: Flac
    description: "Free Lossless Audio Codec"
    media_type: "audio/x-flac"
    extensions: ["flac"]
    signatures:
      - parts:
        - offset: 0
          value: b"fLaC"

  - format: Flif
    description: "Free Lossless Image Format (FLIF)"
    media_type: "image/flif"
    extensions: ["flif"]
    signatures:
      - parts:
        - offset: 0
          value: b"FLIF"

  - format: Flv
    description: "Flash Video (FLV)"
    media_type: "video/x-flv"
    extensions: ["flv"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x46\x4C\x56\x01"

  - format: Gltf
    description: "GL Transmission Format (glTF) binary"
    media_type: "model/gltf-binary"
    extensions: ["glb"]
    signatures:
      - parts:
        - offset: 0
          value: b"glTF"

  - format: Ico
    description: "ICO"
    media_type: "image/x-icon"
    extensions: ["ico", "cur"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x00\x00\x01\x00"

  - format: JavaClass
    description: "Java class"
    media_type: "application/java-vm"
    extensions: ["class"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xCA\xFE\xBA\xBE"

  - format: Lrzip
    description: "Long Range ZIP (lrzip)"
    media_type: "application/x-lrzip"
    extensions: ["lrz"]
    signatures:
      - parts:
        - offset: 0
          value: b"LRZI"

  - format: Lz4
    description: "LZ4"
    media_type: "application/x-lz4"
    extensions: ["lz4"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x04\x22\x4D\x18"

  - format: Lzip
    description: "Lzip"
    media_type: "application/x-lzip"
    extensions: ["lz"]
    signatures:
      - parts:
        - offset: 0
          value: b"LZIP"

  - format: Midi
    description: "Musical Instrument Digital Interface (MIDI)"
    media_type: "audio/midi"
    extensions: ["mid", "midi"]
    signatures:
      - parts:
        - offset: 0
          value: b"MThd"

  - format: Mpc
    description: "Musepack (MPC)"
    media_type: "audio/x-musepack"
    extensions: ["mpc", "mp+", "mpp", "mp"]
    signatures:
      - parts:
        - offset: 0
          value: b"MPCK"
      - parts:
        - offset: 0
          value: b"MP+"

  - format: Mpeg
    description: "MPEG-1 video"
    media_type: "video/mpeg"
    extensions: ["mpg", "mpeg"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x00\x00\x01\xBA"
      - parts:
        - offset: 0
          value: b"\x00\x00\x01\xB3"

  - format: NesRom
    description: "Nintendo Entertainment System ROM"
    media_type: "application/x-nintendo-nes-rom"
    extensions: ["nes"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x4E\x45\x53\x1A"

  - format: OpenExr
    description: "OpenEXR"
    media_type: "image/x-exr"
    extensions: ["exr"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x76\x2F\x31\x01"

  - format: Pcap
    description: "Libpcap"
    media_type: "application/vnd.tcpdump.pcap"
    extensions: ["pcap", "cap", "dmp"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xA1\xB2\xC3\xD4"
      - parts:
        - offset: 0
          value: b"\xD4\xC3\xB2\xA1"

  - format: Pcapng
    description: "Pcap-NG Packet Capture"
    media_type: "application/x-pcapng"
    extensions: ["pcapng"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x0A\x0D\x0D\x0A"

  - format: PhotoshopDocument
    description: "Adobe Photoshop document"
    media_type: "image/vnd.adobe.photoshop"
    extensions: ["psd"]
    signatures:
      - parts:
        - offset: 0
          value: b"8BPS"

  - format: Rpm
    description: "Red Hat Package Manager (RPM) package"
    media_type: "application/x-rpm"
    extensions: ["rpm"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xED\xAB\xEE\xDB"

  - format: Snd
    description: "SouND (SND)"
    media_type: "audio/basic"
    extensions: ["au", "snd"]
    signatures:
      - parts:
        - offset: 0
          value: b".snd"

  - format: Tiff
    description: "Tag Image File Format (TIFF)"
    media_type: "image/tiff"
    extensions: ["tiff", "tif"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x49\x49\x2A\x00"
      - parts:
        - offset: 0
          value: b"\x4D\x4D\x00\x20"

  - format: WavPack
    description: "WavPack"
    media_type: "audio/wavpack"
    extensions: ["wv"]
    signatures:
      - parts:
        - offset: 0
          value: b"wvpk"

  - format: WebAssembly
    description: "WebAssembly binary"
    media_type: "application/wasm"
    extensions: ["wasm"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x00\x61\x73\x6D"

  - format: Wmf
    description: "Windows Metafile (WMF)"
    media_type: "image/wmf"
    extensions: ["wmf"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xD7\xCD\xC6\x9A"
      - parts:
        - offset: 0
          value: b"\x02\x00\x09\x00"
      - parts:
        - offset: 0
          value: b"\x01\x00\x09\x00"

  - format: Woff
    description: "Web Open Font Format (WOFF)"
    media_type: "font/woff"
    extensions: ["woff"]
    signatures:
      - parts:
        - offset: 0
          value: b"wOFF"

  - format: Woff2
    description: "Web Open Font Format 2 (WOFF2)"
    media_type: "font/woff2"
    extensions: ["woff2"]
    signatures:
      - parts:
        - offset: 0
          value: b"wOF2"

  - format: Xar
    description: "eXtensible ARchive format (XAR)"
    media_type: "application/x-xar"
    extensions: ["xar", "xip"]
    signatures:
      - parts:
        - offset: 0
          value: b"xar!"

  - format: Zip
    description: "ZIP"
    media_type: "application/zip"
    extensions: ["zip"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x50\x4B\x03\x04"
      - parts:
        - offset: 0
          value: b"\x50\x4B\x05\x06"
      - parts:
        - offset: 0
          value: b"\x50\x4B\x07\x08"

  - format: Zstandard
    description: "Zstandard"
    media_type: "application/zstd"
    extensions: ["zst"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x28\xB5\x2F\xFD"

  - format: Bzip2
    description: "Bzip2"
    media_type: "application/x-bzip2"
    extensions: ["bz2"]
    signatures:
      - parts:
        - offset: 0
          value: b"BZh"

  - format: JpegXr
    description: "JPEG extended range (JPEG XR)"
    media_type: "image/jxr"
    extensions: ["jxr", "hdp", "wdp"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x49\x49\xBC"

  - format: Mp3
    description: "MPEG-1/2 Audio Layer III (MP3)"
    media_type: "audio/mpeg"
    extensions: ["mp3"]
    signatures:
      - parts:
        - offset: 0
          value: b"ID3"

  - format: Swf
    description: "Small Web Format (SWF)"
    media_type: "application/x-shockwave-flash"
    extensions: ["swf"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x43\x57\x53"
      - parts:
        - offset: 0
          value: b"\x46\x57\x53"

  - format: Aac
    description: "Advanced Audio Coding (AAC)"
    media_type: "audio/aac"
    extensions: ["aac"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xFF\xF1"
      - parts:
        - offset: 0
          value: b"\xFF\xF9"

  - format: Ac3
    description: "Audio Codec 3 (AC-3)"
    media_type: "audio/vnd.dolby.dd-raw"
    extensions: ["ac3"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x0B\x77"

  - format: AppleDiskImage
    description: "Apple Disk Image"
    media_type: "application/x-apple-diskimage"
    extensions: ["dmg", "smi", "img"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x78\x01"

  - format: Bmp
    description: "Windows Bitmap (BMP)"
    media_type: "image/bmp"
    extensions: ["bmp", "dib"]
    signatures:
      - parts:
        - offset: 0
          value: b"BM"

  - format: Compress
    description: "Unix compress"
    media_type: "application/x-compress"
    extensions: ["Z"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x1F\xA0"
      - parts:
        - offset: 0
          value: b"\x1F\x9D"

  - format: Exe
    description: "Windows Executable"
    media_type: "application/x-msdownload"
    extensions: ["exe", "dll"]
    signatures:
      - parts:
        - offset: 0
          value: b"MZ"

  - format: Gzip
    description: "Gzip"
    media_type: "application/gzip"
    extensions: ["gz"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x1F\x8B"
}
