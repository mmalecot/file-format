#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[macro_use]
mod macros;

file_format! {
  - format: Dicom
    media_type: "application/dicom"
    extensions: ["dcm"]
    signatures:
      - parts:
        - offset: 128
          value: b"\x44\x49\x43\x4D"

  - format: Gzip
    media_type: "application/gzip"
    extensions: ["gz"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x1F\x8B"

  - format: JavaClass
    media_type: "application/java-vm"
    extensions: ["class"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xCA\xFE\xBA\xBE"

  - format: Pdf
    media_type: "application/pdf"
    extensions: ["pdf"]
    signatures:
      - parts:
        - offset: 0
          value: b"%PDF-"

  - format: Dex
    media_type: "application/vnd.android.dex"
    extensions: ["dex"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x64\x65\x78\x0a"

  - format: Rar
    media_type: "application/vnd.rar"
    extensions: ["rar"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x52\x61\x72\x21\x1A\x07\x00"
      - parts:
        - offset: 0
          value: b"\x52\x61\x72\x21\x1A\x07\x01\x00"

  - format: Sqlite3
    media_type: "application/vnd.sqlite3"
    extensions: ["sqlite", "sqlite3", "sqlitedb", "db"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x53\x51\x4C\x69\x74\x65\x20\x66\x6F\x72\x6D\x61\x74\x20\x33\x00"

  - format: Pcap
    media_type: "application/vnd.tcpdump.pcap"
    extensions: ["pcap", "cap", "dmp"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xA1\xB2\xC3\xD4"
      - parts:
        - offset: 0
          value: b"\xD4\xC3\xB2\xA1"

  - format: WebAssembly
    media_type: "application/wasm"
    extensions: ["wasm"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x00\x61\x73\x6D"

  - format: SevenZip
    media_type: "application/x-7z-compressed"
    extensions: ["7z"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x37\x7A\xBC\xAF\x27\x1C"

  - format: AppleDiskImage
    media_type: "application/x-apple-diskimage"
    extensions: ["dmg", "smi", "img"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x78\x01"

  - format: Blender
    media_type: "application/x-blender"
    extensions: ["blend"]
    signatures:
      - parts:
        - offset: 0
          value: b"BLENDER"

  - format: Bzip2
    media_type: "application/x-bzip2"
    extensions: ["bz2"]
    signatures:
      - parts:
        - offset: 0
          value: b"BZh"

  - format: Compress
    media_type: "application/x-compress"
    extensions: ["Z"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x1F\xA0"
      - parts:
        - offset: 0
          value: b"\x1F\x9D"

  - format: Deb
    media_type: "application/x-deb"
    extensions: ["deb"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x21\x3C\x61\x72\x63\x68\x3E\x0A"

  - format: Elf
    media_type: "application/x-executable"
    extensions: ["elf", "so"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x7F\x45\x4C\x46"

  - format: GbRom
    media_type: "application/x-gameboy-rom"
    extensions: ["gb"]
    signatures:
      - parts:
        - offset: 0x104
          value: b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B"

  - format: GbaRom
    media_type: "application/x-gba-rom"
    extensions: ["gba"]
    signatures:
      - parts:
        - offset: 4
          value: b"\x24\xFF\xAE\x51\x69\x9A\xA2\x21"

  - format: GoogleChromeExtension
    media_type: "application/x-google-chrome-extension"
    extensions: ["crx"]
    signatures:
      - parts:
        - offset: 0
          value: b"Cr24"

  - format: InDesignDocument
    media_type: "application/x-indesign"
    extensions: ["indd"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x06\x06\xED\xF5\xD8\x1D\x46\xE5\xBD\x31\xEF\xE7\xFE\x74\xB7\x1D"

  - format: Iso
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

  - format: Lrzip
    media_type: "application/x-lrzip"
    extensions: ["lrz"]
    signatures:
      - parts:
        - offset: 0
          value: b"LRZI"

  - format: Lz4
    media_type: "application/x-lz4"
    extensions: ["lz4"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x04\x22\x4D\x18"

  - format: Lzip
    media_type: "application/x-lzip"
    extensions: ["lz"]
    signatures:
      - parts:
        - offset: 0
          value: b"LZIP"

  - format: Lzop
    media_type: "application/x-lzop"
    extensions: ["lzo"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x89\x4C\x5A\x4F\x00\x0D\x0A\x1A\x0A"

  - format: Exe
    media_type: "application/x-msdownload"
    extensions: ["exe", "dll"]
    signatures:
      - parts:
        - offset: 0
          value: b"MZ"

  - format: N64Rom
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
    media_type: "application/x-nintendo-ds-rom"
    extensions: ["nds"]
    signatures:
      - parts:
        - offset: 0xC0
          value: b"\x24\xFF\xAE\x51\x69\x9A\xA2\x21"
      - parts:
        - offset: 0xC0
          value: b"\xC8\x60\x4F\xE2\x01\x70\x8F\xE2"

  - format: NesRom
    media_type: "application/x-nintendo-nes-rom"
    extensions: ["nes"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x4E\x45\x53\x1A"

  - format: Pcapng
    media_type: "application/x-pcapng"
    extensions: ["pcapng"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x0A\x0D\x0D\x0A"

  - format: Rpm
    media_type: "application/x-rpm"
    extensions: ["rpm"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xED\xAB\xEE\xDB"

  - format: Swf
    media_type: "application/x-shockwave-flash"
    extensions: ["swf"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x43\x57\x53"
      - parts:
        - offset: 0
          value: b"\x46\x57\x53"

  - format: Tar
    media_type: "application/x-tar"
    extensions: ["tar"]
    signatures:
      - parts:
        - offset: 257
          value: b"\x75\x73\x74\x61\x72\x00\x30\x30"
      - parts:
        - offset: 257
          value: b"\x75\x73\x74\x61\x72\x20\x20\x00"

  - format: Vdi
    media_type: "application/x-virtualbox-vdi"
    extensions: ["vdi"]
    signatures:
      - parts:
        - offset: 0
          value: b"<<< Oracle VM VirtualBox Disk Image >>>"

  - format: Xar
    media_type: "application/x-xar"
    extensions: ["xar", "xip"]
    signatures:
      - parts:
        - offset: 0
          value: b"xar!"

  - format: Xz
    media_type: "application/x-xz"
    extensions: ["xz"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xFD\x37\x7A\x58\x5A\x00"

  - format: Zip
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
    media_type: "application/zstd"
    extensions: ["zst"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x28\xB5\x2F\xFD"

  - format: Aac
    media_type: "audio/aac"
    extensions: ["aac"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xFF\xF1"
      - parts:
        - offset: 0
          value: b"\xFF\xF9"

  - format: Aiff
    media_type: "audio/aiff"
    extensions: ["aif", "aiff"]
    signatures:
      - parts:
        - offset: 0
          value: b"FORM"
        - offset: 8
          value: b"AIFF"

  - format: Amr
    media_type: "audio/amr"
    extensions: ["amr", "3ga"]
    signatures:
      - parts:
        - offset: 0
          value: b"#!AMR"

  - format: Snd
    media_type: "audio/basic"
    extensions: ["au", "snd"]
    signatures:
      - parts:
        - offset: 0
          value: b".snd"

  - format: Midi
    media_type: "audio/midi"
    extensions: ["mid", "midi"]
    signatures:
      - parts:
        - offset: 0
          value: b"MThd"

  - format: Mp3
    media_type: "audio/mpeg"
    extensions: ["mp3"]
    signatures:
      - parts:
        - offset: 0
          value: b"ID3"

  - format: OggAudio
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
        - offset: 29
          value: b"FLAC"
      - parts:
        - offset: 0
          value: b"OggS"
        - offset: 28
          value: b"Speex"

  - format: Ac3
    media_type: "audio/vnd.dolby.dd-raw"
    extensions: ["ac3"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x0B\x77"

  - format: Wave
    media_type: "audio/vnd.wave"
    extensions: ["wav"]
    signatures:
      - parts:
        - offset: 0
          value: b"RIFF"
        - offset: 8
          value: b"WAVE"

  - format: WavPack
    media_type: "audio/wavpack"
    extensions: ["wv"]
    signatures:
      - parts:
        - offset: 0
          value: b"wvpk"

  - format: Flac
    media_type: "audio/x-flac"
    extensions: ["flac"]
    signatures:
      - parts:
        - offset: 0
          value: b"fLaC"

  - format: M4a
    media_type: "audio/x-m4a"
    extensions: ["m4a"]
    signatures:
      - parts:
        - offset: 4
          value: b"ftypM4A"

  - format: Mpc
    media_type: "audio/x-musepack"
    extensions: ["mpc", "mp+", "mpp", "mp"]
    signatures:
      - parts:
        - offset: 0
          value: b"MP+"
      - parts:
        - offset: 0
          value: b"MPCK"

  - format: OpenType
    media_type: "font/otf"
    extensions: ["otf"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x4F\x54\x54\x4F\x00"

  - format: TrueType
    media_type: "font/ttf"
    extensions: ["ttf"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x00\x01\x00\x00\x00"

  - format: Woff
    media_type: "font/woff"
    extensions: ["woff"]
    signatures:
      - parts:
        - offset: 0
          value: b"wOFF"

  - format: Woff2
    media_type: "font/woff2"
    extensions: ["woff2"]
    signatures:
      - parts:
        - offset: 0
          value: b"wOF2"

  - format: Avif
    media_type: "image/avif"
    extensions: ["avif"]
    signatures:
      - parts:
        - offset: 4
          value: b"ftypavif"

  - format: Bmp
    media_type: "image/bmp"
    extensions: ["bmp", "dib"]
    signatures:
      - parts:
        - offset: 0
          value: b"BM"

  - format: Bpg
    media_type: "image/bpg"
    extensions: ["bpg"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x42\x50\x47\xFB"

  - format: Cineon
    media_type: "image/cineon"
    extensions: ["cin"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x80\x2A\x5F\xD7"

  - format: Fits
    media_type: "image/fits"
    extensions: ["fits", "fit", "fts"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x53\x49\x4D\x50\x4C\x45\x20\x20\x3D\x20"

  - format: Flif
    media_type: "image/flif"
    extensions: ["flif"]
    signatures:
      - parts:
        - offset: 0
          value: b"FLIF"

  - format: Gif
    media_type: "image/gif"
    extensions: ["gif"]
    signatures:
      - parts:
        - offset: 0
          value: b"GIF87a"
      - parts:
        - offset: 0
          value: b"GIF89a"

  - format: Heif
    media_type: "image/heic"
    extensions: ["heic"]
    signatures:
      - parts:
        - offset: 4
          value: b"ftypheic"
      - parts:
        - offset: 4
          value: b"ftypheix"

  - format: AppleIconImage
    media_type: "image/icns"
    extensions: ["icns"]
    signatures:
      - parts:
        - offset: 0
          value: b"icns"

  - format: Jp2
    media_type: "image/jp2"
    extensions: ["jp2"]
    signatures:
      - parts:
        - offset: 16
          value: b"ftypjp2"

  - format: Jpeg
    media_type: "image/jpeg"
    extensions: ["jpg", "jpeg"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xFF\xD8\xFF\xDB"
      - parts:
        - offset: 0
          value: b"\xFF\xD8\xFF\xE0\x00\x10\x4A\x46\x49\x46\x00\x01"
      - parts:
        - offset: 0
          value: b"\xFF\xD8\xFF\xEE"
      - parts:
        - offset: 0
          value: b"\xFF\xD8\xFF\xE1"
        - offset: 6
          value: b"\x45\x78\x69\x66\x00\x00"

  - format: JpegXr
    media_type: "image/jxr"
    extensions: ["jxr", "hdp", "wdp"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x49\x49\xBC"

  - format: JpegXl
    media_type: "image/jxl"
    extensions: ["jxl"]
    signatures:
      - parts:
        - offset: 0
          value: b"\xFF\x0A"
      - parts:
        - offset: 0
          value: b"\x00\x00\x00\x0C\x4A\x58\x4C\x20\x0D\x0A\x87\x0A"

  - format: Png
    media_type: "image/png"
    extensions: ["png"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"

  - format: Tiff
    media_type: "image/tiff"
    extensions: ["tiff", "tif"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x49\x49\x2A\x00"
      - parts:
        - offset: 0
          value: b"\x4D\x4D\x00\x20"

  - format: PhotoshopDocument
    media_type: "image/vnd.adobe.photoshop"
    extensions: ["psd"]
    signatures:
      - parts:
        - offset: 0
          value: b"8BPS"

  - format: WebP
    media_type: "image/webp"
    extensions: ["webp"]
    signatures:
      - parts:
        - offset: 0
          value: b"RIFF"
        - offset: 8
          value: b"WEBP"

  - format: Dpx
    media_type: "image/x-dpx"
    extensions: ["dpx"]
    signatures:
      - parts:
        - offset: 0
          value: b"SDPX"
      - parts:
        - offset: 0
          value: b"XPDS"

  - format: OpenExr
    media_type: "image/x-exr"
    extensions: ["exr"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x76\x2F\x31\x01"

  - format: Ico
    media_type: "image/x-icon"
    extensions: ["ico"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x00\x00\x01\x00"

  - format: Xcf
    media_type: "image/x-xcf"
    extensions: ["xcf"]
    signatures:
      - parts:
        - offset: 0
          value: b"gimp xcf"

  - format: Gltf
    media_type: "model/gltf-binary"
    extensions: ["glb"]
    signatures:
      - parts:
        - offset: 0
          value: b"glTF"

  - format: ICalendar
    media_type: "text/calendar"
    extensions: ["ics", "ical", "ifb", "icalendar"]
    signatures:
      - parts:
        - offset: 0
          value: b"BEGIN:VCALENDAR"

  - format: VCard
    media_type: "text/vcard"
    extensions: ["vcf", "vcard"]
    signatures:
      - parts:
        - offset: 0
          value: b"BEGIN:VCARD"

  - format: Xml
    media_type: "text/xml"
    extensions: ["xml"]
    signatures:
      - parts:
        - offset: 0
          value: b"<?xml "

  - format: ThirdGpp
    media_type: "video/3gpp"
    extensions: ["3gp"]
    signatures:
      - parts:
        - offset: 4
          value: b"ftyp3gp"

  - format: ThirdGpp2
    media_type: "video/3gpp2"
    extensions: ["3g2"]
    signatures:
      - parts:
        - offset: 4
          value: b"ftyp3g2"

  - format: Avi
    media_type: "video/avi"
    extensions: ["avi"]
    signatures:
      - parts:
        - offset: 0
          value: b"RIFF"
        - offset: 8
          value: b"\x41\x56\x49\x20"

  - format: Mp4
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

  - format: Mpeg
    media_type: "video/mpeg"
    extensions: ["mpg", "mpeg"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x00\x00\x01\xBA"
      - parts:
        - offset: 0
          value: b"\x00\x00\x01\xB3"

  - format: OggVideo
    media_type: "video/ogg"
    extensions: ["ogv"]
    signatures:
      - parts:
        - offset: 0
          value: b"OggS"
        - offset: 29
          value: b"theora"

  - format: QuickTime
    media_type: "video/quicktime"
    extensions: ["mov", "qt"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x00\x00\x00\x14"
        - offset: 4
          value: b"ftypqt"

  - format: WebM
    media_type: "video/webm"
    extensions: ["webm"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x1A\x45\xDF\xA3"
        - offset: 24
          value: b"webm"

  - format: Flv
    media_type: "video/x-flv"
    extensions: ["flv"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x46\x4C\x56\x01"

  - format: M4v
    media_type: "video/x-m4v"
    extensions: ["m4v"]
    signatures:
      - parts:
        - offset: 4
          value: b"ftypM4V"

  - format: Matroska
    media_type: "video/x-matroska"
    extensions: ["mkv"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x1A\x45\xDF\xA3"
        - offset: 24
          value: b"matroska"

  - format: Wmv
    media_type: "video/x-ms-asf"
    extensions: ["wmv", "wm", "asf"]
    signatures:
      - parts:
        - offset: 0
          value: b"\x30\x26\xB2\x75\x8E\x66\xCF\x11\xA6\xD9"
}
