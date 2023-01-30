//! Signatures.

signatures! {
    // 39 bytes
    format = VirtualboxVirtualDiskImage
    value = b"<<< Oracle VM VirtualBox Disk Image >>>"

    // 37 bytes
    format = PgpPrivateKeyBlock
    value = b"-----BEGIN PGP PRIVATE KEY BLOCK-----"

    // 36 bytes
    format = PgpPublicKeyBlock
    value = b"-----BEGIN PGP PUBLIC KEY BLOCK-----"

    // 35 bytes
    format = PemCertificateSigningRequest
    value = b"-----BEGIN CERTIFICATE REQUEST-----"

    // 34 bytes
    format = PgpSignedMessage
    value = b"-----BEGIN PGP SIGNED MESSAGE-----"

    // 33 bytes
    format = PemPrivateKey
    value = b"-----BEGIN ECDSA PRIVATE KEY-----"
    value = b"-----BEGIN DSA PRIVATE KEY-----"
    value = b"-----BEGIN DSA PRIVATE KEY-----"
    value = b"-----BEGIN EC PRIVATE KEY-----"
    value = b"-----BEGIN PRIVATE KEY-----"

    // 32 bytes
    format = Sketchup
    value =
        b"\xFF\xFE\xFF\x0E\x53\x00\x6B\x00\x65\x00\x74\x00\x63\x00\x68\x00",
        b"\x55\x00\x70\x00\x20\x00\x4D\x00\x6F\x00\x64\x00\x65\x00\x6C\x00" offset = 16

    // 29 bytes
    format = FlexibleImageTransportSystem
    value =
        b"\x49\x4D\x50\x4C\x45\x20\x20\x3D\x20\x20\x20\x20\x20\x20\x20",
        b"\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x54" offset = 15

    format = PgpSignature
    value = b"-----BEGIN PGP SIGNATURE-----"

    // 28 bytes
    format = Vcalendar
    value = b"BEGIN:VCALENDAR\r\nVERSION:1.0"
    value = b"BEGIN:VCALENDAR\nVERSION:1.0"
    value = b"BEGIN:VCALENDAR\rVERSION:1.0"

    // 27 bytes
    format = PemCertificate
    value = b"-----BEGIN CERTIFICATE-----"

    format = PgpMessage
    value = b"-----BEGIN PGP MESSAGE-----"

    // 24 bytes
    format = ClojureScript
    value = b"#!/usr/local/bin/clojure"
    value = b"#!/usr/bin/env clojure"
    value = b"#!/usr/local/bin/clj"
    value = b"#!/usr/bin/clojure"
    value = b"#!/usr/bin/env clj"
    value = b"#!/usr/bin/clj"

    // 23 bytes
    format = PythonScript
    value = b"#!/usr/local/bin/python"
    value = b"#!/usr/bin/env python"
    value = b"#!/usr/bin/python"

    // 22 bytes
    format = CreativeVoice
    value = b"Creative Voice File\x1A\x1A\x00"

    format = ToolCommandLanguageScript
    value = b"#!/usr/local/bin/tclsh"
    value = b"#!/usr/local/bin/wish"
    value = b"#!/usr/local/bin/tcl"
    value = b"#!/usr/bin/env tclsh"
    value = b"#!/usr/bin/env wish"
    value = b"#!/usr/bin/env tcl"
    value = b"#!/usr/bin/tclsh"
    value = b"#!/usr/bin/wish"
    value = b"#!/usr/bin/tcl"

    // 21 bytes
    format = DebianBinaryPackage
    value = b"!<arch>\ndebian-binary"

    format = RubyScript
    value = b"#!/usr/local/bin/ruby"
    value = b"#!/usr/bin/env ruby"
    value = b"#!/usr/bin/ruby"

    format = ShellScript
    value = b"#!/usr/local/bin/bash"
    value = b"#!/usr/local/bin/fish"
    value = b"#!/usr/local/bin/tcsh"
    value = b"#!/usr/local/bin/ash"
    value = b"#!/usr/local/bin/zsh"
    value = b"#!/usr/bin/env bash"
    value = b"#!/usr/bin/env fish"
    value = b"#!/usr/bin/env zsh"
    value = b"#!/usr/local/bash"
    value = b"#!/usr/local/tcsh"
    value = b"#!/usr/bin/bash"
    value = b"#!/usr/bin/fish"
    value = b"#!/usr/bin/tcsh"
    value = b"#!/usr/bin/zsh"
    value = b"#!/bin/bash"
    value = b"#!/bin/tcsh"
    value = b"#!/bin/zsh"
    value = b"#!/bin/ash"
    value = b"#!/bin/csh"
    value = b"#!/bin/ksh"
    value = b"#!/bin/sh"

    // 20 bytes
    format = LuaScript
    value = b"#!/usr/local/bin/lua"
    value = b"#!/usr/bin/env lua"
    value = b"#!/usr/bin/lua"

    format = WindowsShortcut
    value = b"\x4C\x00\x00\x00\x01\x14\x02\x00\x00\x00\x00\x00\xC0\x00\x00\x00\x00\x00\x00\x46"

    // 19 bytes
    format = PerlScript
    value = b"#!/usr/bin/env perl"
    value = b"#!/usr/bin/perl"

    // 17 bytes
    format = EncapsulatedPostscript
    value = b"%!PS-Adobe-", b" EPSF-" offset = 14
    value = b"\xC5\xD0\xD3\xC6"

    // 16 bytes
    format = AdobeIndesignDocument
    value = b"\x06\x06\xED\xF5\xD8\x1D\x46\xE5\xBD\x31\xEF\xE7\xFE\x74\xB7\x1D"

    format = FastTracker2ExtendedModule
    value = b"Extended Module:"

    format = MacosAlias
    value = b"\x62\x6F\x6F\x6B\x00\x00\x00\x00\x6D\x61\x72\x6B\x00\x00\x00\x00"

    format = Sqlite3
    value = b"\x53\x51\x4C\x69\x74\x65\x20\x66\x6F\x72\x6D\x61\x74\x20\x33\x00"

    format = WindowsMediaVideo
    value = b"\x30\x26\xB2\x75\x8E\x66\xCF\x11\xA6\xD9\x00\xAA\x00\x62\xCE\x6C"

    // 15 bytes
    format = FujifilmRaw
    value = b"FUJIFILMCCD-RAW"

    format = Icalendar
    value = b"BEGIN:VCALENDAR"

    format = MicrosoftAccess2007Database
    value = b"Standard ACE DB" offset = 4

    format = MicrosoftAccessDatabase
    value = b"Standard Jet DB" offset = 4

    // 14 bytes
    format = HypertextMarkupLanguage
    value = b"<!DOCTYPE HTML"
    value = b"<!DOCTYPE html"
    value = b"<!doctype HTML"
    value = b"<!doctype html"

    format = Latex
    value = b"\\documentclass"
    value = b"\\documentstyle"

    format = MaterialExchangeFormat
    value = b"\x06\x0E\x2B\x34\x02\x05\x01\x01\x0D\x01\x02\x01\x01\x02"

    // 12 bytes
    format = AnimatedPortableNetworkGraphics
    value = b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A", b"acTL" offset = 0x25

    format = Djvu
    value = b"AT&TFORM", b"DJVM" offset = 12
    value = b"AT&TFORM", b"DJVU" offset = 12
    value = b"AT&TFORM", b"DJVI" offset = 12
    value = b"AT&TFORM", b"THUM" offset = 12

    format = JpegXl
    value = b"\x00\x00\x00\x0C\x4A\x58\x4C\x20\x0D\x0A\x87\x0A"
    value = b"\xFF\x0A"

    format = KhronosTexture
    value = b"\xAB\x4B\x54\x58\x20\x31\x31\xBB\x0D\x0A\x1A\x0A"

    format = KhronosTexture2
    value = b"\xAB\x4B\x54\x58\x20\x32\x30\xBB\x0D\x0A\x1A\x0A"

    format = MatroskaVideo
    value = b"\x1A\x45\xDF\xA3"

    format = OggOpus
    value = b"OggS", b"OpusHead" offset = 28

    format = PanasonicRaw
    value = b"\x49\x49\x55\x00\x18\x00\x00\x00\x88\xE7\x74\xD8"

    // 11 bytes
    format = BittorrentFile
    value = b"d8:announce"

    format = OggSpeex
    value = b"OggS", b"Speex  " offset = 28

    format = OggTheora
    value = b"OggS", b"\x80\x74\x68\x65\x6F\x72\x61" offset = 28

    format = OggVorbis
    value = b"OggS", b"\x01\x76\x6F\x72\x62\x69\x73" offset = 28

    format = RadianceHdr
    value = b"\x23\x3F\x52\x41\x44\x49\x41\x4E\x43\x45\x0A"

    format = Vcard
    value = b"BEGIN:VCARD"

    // 10 bytes
    format = OggMedia
    value = b"OggS", b"\x01\x76\x69\x64\x65\x6F" offset = 28

    format = Snappy
    value = b"\xFF\x06\x00\x00\x73\x4E\x61\x50\x70\x59"

    // 9 bytes
    format = GameBoyColorRom
    value = b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B" offset = 0x104, b"\x80" offset = 0x143
    value = b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B" offset = 0x104, b"\xC0" offset = 0x143

    format = Lzop
    value = b"\x89\x4C\x5A\x4F\x00\x0D\x0A\x1A\x0A"

    format = MicrosoftVirtualHardDisk
    value = b"connectix"

    format = OggFlac
    value = b"OggS", b"\x7F\x46\x4C\x41\x43" offset = 28

    format = OlympusRawFormat
    value = b"\x49\x49\x52\x4F\x08\x00\x00\x00\x18"

    // 8 bytes
    format = AudioInterchangeFileFormat
    value = b"FORM", b"AIFF" offset = 8
    value = b"FORM", b"AIFC" offset = 8

    format = AudioVideoInterleave
    value = b"RIFF", b"\x41\x56\x49\x20" offset = 8

    format = Av1ImageFileFormat
    value = b"ftypavif" offset = 4

    format = Av1ImageFileFormatSequence
    value = b"ftypavis" offset = 4

    format = CompoundFileBinary
    value = b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1"

    format = DalvikExecutable
    value = b"\x64\x65\x78\x0A\x30\x33\x35\x00"

    format = ExperimentalComputingFacility
    value = b"gimp xcf"

    format = GameBoyAdvanceRom
    value = b"\x24\xFF\xAE\x51\x69\x9A\xA2\x21" offset = 4

    format = GameBoyRom
    value = b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B" offset = 0x104

    format = HighEfficiencyImageCoding
    value = b"ftypheic" offset = 4
    value = b"ftypheix" offset = 4

    format = HighEfficiencyImageCodingSequence
    value = b"ftyphevc" offset = 4
    value = b"ftyphevx" offset = 4

    format = HighEfficiencyImageFileFormat
    value = b"ftypmif1" offset = 4

    format = HighEfficiencyImageFileFormatSequence
    value = b"ftypmsf1" offset = 4

    format = Jpeg2000Part1
    value = b"ftypjp2 " offset = 16

    format = Jpeg2000Part2
    value = b"ftypjpx " offset = 16

    format = Jpeg2000Part3
    value = b"ftypmjp2" offset = 16

    format = Jpeg2000Part6
    value = b"ftypjpm " offset = 16

    format = MicrosoftVirtualHardDisk2
    value = b"vhdxfile"

    format = Mobipocket
    value = b"BOOKMOBI" offset = 60

    format = Mpeg4Part14Video
    value = b"ftypavc1" offset = 4
    value = b"ftypdash" offset = 4
    value = b"ftypiso2" offset = 4
    value = b"ftypiso3" offset = 4
    value = b"ftypiso4" offset = 4
    value = b"ftypiso5" offset = 4
    value = b"ftypiso6" offset = 4
    value = b"ftypisom" offset = 4
    value = b"ftypmmp4" offset = 4
    value = b"ftypmp41" offset = 4
    value = b"ftypmp42" offset = 4
    value = b"ftypmp4v" offset = 4
    value = b"ftypmp71" offset = 4
    value = b"ftypMSNV" offset = 4
    value = b"ftypNDAS" offset = 4
    value = b"ftypNDSC" offset = 4
    value = b"ftypNDSH" offset = 4
    value = b"ftypNDSM" offset = 4
    value = b"ftypNDSP" offset = 4
    value = b"ftypNDSS" offset = 4
    value = b"ftypNDXC" offset = 4
    value = b"ftypNDXH" offset = 4
    value = b"ftypNDXM" offset = 4
    value = b"ftypNDXP" offset = 4

    format = NikonElectronicFile
    value = b"\x4D\x4D\x00\x2A", b"\x1C\x00\xFE\x00" offset = 8
    value = b"\x4D\x4D\x00\x2A", b"\x1F\x00\x0B\x00" offset = 8
    value = b"\x49\x49\x2A\x00", b"\x1C\x00\xFE\x00" offset = 8
    value = b"\x49\x49\x2A\x00", b"\x1F\x00\x0B\x00" offset = 8

    format = Nintendo64Rom
    value = b"\x80\x37\x12\x40\x00\x00\x00\x0F"
    value = b"\x37\x80\x40\x12\x00\x00\x0F\x00"
    value = b"\x12\x40\x80\x37\x00\x0F\x00\x00"
    value = b"\x40\x12\x37\x80\x0F\x00\x00\x00"

    format = NintendoDsRom
    value = b"\x24\xFF\xAE\x51\x69\x9A\xA2\x21" offset = 0xC0
    value = b"\xC8\x60\x4F\xE2\x01\x70\x8F\xE2" offset = 0xC0

    format = PortableNetworkGraphics
    value = b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"

    format = QualcommPureVoice
    value = b"RIFF", b"QLCM" offset = 8

    format = RoshalArchive
    value = b"\x52\x61\x72\x21\x1A\x07\x01\x00"
    value = b"\x52\x61\x72\x21\x1A\x07\x00"

    format = TapeArchive
    value = b"\x75\x73\x74\x61\x72\x00\x30\x30" offset = 257
    value = b"\x75\x73\x74\x61\x72\x20\x20\x00" offset = 257

    format = WaveformAudio
    value = b"RIFF", b"WAVE" offset = 8

    format = Webp
    value = b"RIFF", b"WEBP" offset = 8

    format = WindowsAnimatedCursor
    value = b"RIFF", b"ACON" offset = 8

    // 7 bytes
    format = AdobeFlashPlayerAudio
    value = b"ftypF4A" offset = 4

    format = AdobeFlashPlayerAudiobook
    value = b"ftypF4B" offset = 4

    format = AdobeFlashPlayerProtectedVideo
    value = b"ftypF4P" offset = 4

    format = AdobeFlashPlayerVideo
    value = b"ftypF4V" offset = 4

    format = AppleItunesAudio
    value = b"ftypM4A" offset = 4

    format = AppleItunesAudiobook
    value = b"ftypM4B" offset = 4

    format = AppleItunesProtectedAudio
    value = b"ftypM4P" offset = 4

    format = AppleItunesVideo
    value = b"ftypM4V" offset = 4

    format = Blender
    value = b"BLENDER"

    format = CanonRaw3
    value = b"ftypcrx" offset = 4

    format = MetaInformationEncapsulation
    value = b"\x7E\x10\x04", b"\x30\x4D\x49\x45" offset = 4
    value = b"\x7E\x18\x04", b"\x30\x4D\x49\x45" offset = 4

    format = Mp3Url
    value = b"#EXTM3U"

    format = SonyMovie
    value = b"ftypmqt" offset = 4

    format = ThirdGenerationPartnershipProject
    value = b"ftyp3gp" offset = 4

    format = ThirdGenerationPartnershipProject2
    value = b"ftyp3g2" offset = 4

    format = UnixArchiver
    value = b"!<arch>"

    // 6 bytes
    format = ApacheArrowColumnar
    value = b"ARROW1"

    format = AppleQuicktime
    value = b"ftypqt" offset = 4
    value = b"moov" offset = 4
    value = b"mdat" offset = 4
    value = b"wide" offset = 4
    value = b"skip" offset = 4
    value = b"free" offset = 4

    format = CanonRaw2
    value = b"\x4D\x4D\x00\x2A", b"CR" offset = 8
    value = b"\x49\x49\x2A\x00", b"CR" offset = 8

    format = ExtensibleMarkupLanguage
    value = b"<?xml "

    format = GraphicsInterchangeFormat
    value = b"GIF87a"
    value = b"GIF89a"

    format = SevenZip
    value = b"\x37\x7A\xBC\xAF\x27\x1C"
    value = b"\x37\x7A\xBC\xAF\x27\x1C"

    format = Xz
    value = b"\xFD\x37\x7A\x58\x5A\x00"

    // 5 bytes
    format = AdaptiveMultiRate
    value = b"#!AMR"

    format = EmbeddedOpentype
    value = b"\x00\x00\x01" offset = 8, b"\x4C\x50" offset = 34
    value = b"\x01\x00\x02" offset = 8, b"\x4C\x50" offset = 34
    value = b"\x02\x00\x02" offset = 8, b"\x4C\x50" offset = 34

    format = GoogleDraco
    value = b"DRACO"

    format = Iso9660
    value = b"CD001" offset = 0x8001
    value = b"CD001" offset = 0x8801
    value = b"CD001" offset = 0x9001

    format = Larc
    value = b"-lz2-" offset = 2
    value = b"-lz3-" offset = 2
    value = b"-lz4-" offset = 2
    value = b"-lz5-" offset = 2
    value = b"-lz7-" offset = 2
    value = b"-lz8-" offset = 2
    value = b"-lzs-" offset = 2

    format = Lha
    value = b"-lh0-" offset = 2
    value = b"-lh1-" offset = 2
    value = b"-lh2-" offset = 2
    value = b"-lh3-" offset = 2
    value = b"-lh4-" offset = 2
    value = b"-lh5-" offset = 2
    value = b"-lh6-" offset = 2
    value = b"-lh7-" offset = 2
    value = b"-lh8-" offset = 2
    value = b"-lhd-" offset = 2

    format = MachO
    value = b"\xCA\xFE\xBA\xBE", b"\x01" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x02" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x03" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x04" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x05" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x06" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x07" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x08" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x09" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x0A" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x0B" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x0C" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x0D" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x0E" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x0F" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x10" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x11" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\x12" offset = 7
    value = b"\xCA\xFE\xBA\xBE", b"\xFF" offset = 7
    value = b"\xFE\xED\xFA\xCE"
    value = b"\xFE\xED\xFA\xCF"
    value = b"\xCE\xFA\xED\xFE"
    value = b"\xCF\xFA\xED\xFE"

    format = Opentype
    value = b"\x4F\x54\x54\x4F\x00"

    format = Pmarc
    value = b"-pc1-" offset = 2
    value = b"-pm0-" offset = 2
    value = b"-pm1-" offset = 2
    value = b"-pm2-" offset = 2
    value = b"-pms-" offset = 2

    format = PortableDocumentFormat
    value = b"%PDF-"

    format = RichTextFormat
    value = b"\x7B\x5C\x72\x74\x66"

    format = Truetype
    value = b"\x00\x01\x00\x00\x00"

    // 4 bytes
    format = AdobePhotoshopDocument
    value = b"8BPS"

    format = Alz
    value = b"\x41\x4C\x5A\x01"

    format = AndroidBinaryXml
    value = b"\x03\x00\x08\x00"

    format = AndroidCompiledResources
    value = b"\x02\x00\x0C\x00"

    format = ApacheParquet
    value = b"PAR1"

    format = AppleIconImage
    value = b"icns"

    format = Au
    value = b".snd"

    format = AutocadDrawing
    value = b"AC10"

    format = BetterPortableGraphics
    value = b"\x42\x50\x47\xFB"

    format = Cabinet
    value = b"MSCF"
    value = b"ISc("

    format = Cineon
    value = b"\x80\x2A\x5F\xD7"

    format = DigitalImagingAndCommunicationsInMedicine
    value = b"\x44\x49\x43\x4D" offset = 128

    format = DigitalPictureExchange
    value = b"SDPX"
    value = b"XPDS"

    format = ExecutableAndLinkableFormat
    value = b"\x7F\x45\x4C\x46"

    format = ExtensibleArchive
    value = b"xar!"

    format = FlashVideo
    value = b"\x46\x4C\x56\x01"

    format = FreeLosslessAudioCodec
    value = b"fLaC"

    format = FreeLosslessImageFormat
    value = b"FLIF"

    format = GlTransmissionFormatBinary
    value = b"glTF"

    format = GoogleChromeExtension
    value = b"Cr24"

    format = IccProfile
    value = b"acsp" offset = 36

    format = ImpulseTrackerModule
    value = b"IMPM"

    format = JavaClass
    value = b"\xCA\xFE\xBA\xBE"

    format = JavaKeystore
    value = b"\xFE\xED\xFE\xED"

    format = JpegLs
    value = b"\xFF\xD8\xFF\xF7"

    format = LempelZivFiniteStateEntropy
    value = b"bvx-"
    value = b"bvx1"
    value = b"bvx2"
    value = b"bvxn"

    format = LongRangeZip
    value = b"LRZI"

    format = LuaBytecode
    value = b"\x1B\x4C\x75\x61"

    format = Lz4
    value = b"\x04\x22\x4D\x18"

    format = Lzip
    value = b"LZIP"

    format = MicrosoftCompiledHtmlHelp
    value = b"ITSF"

    format = MicrosoftDirectDrawSurface
    value = b"DDS "

    format = MonkeysAudio
    value = b"MAC "

    format = Mpeg1Video
    value = b"\x00\x00\x01\xBA"
    value = b"\x00\x00\x01\xB3"

    format = Musepack
    value = b"MPCK"
    value = b"MP+"

    format = MusicalInstrumentDigitalInterface
    value = b"MThd"

    format = NintendoEntertainmentSystemRom
    value = b"\x4E\x45\x53\x1A"

    format = OggMultiplexedMedia
    value = b"OggS"

    format = Openexr
    value = b"\x76\x2F\x31\x01"

    format = OptimizedDalvikExecutable
    value = b"dey\n"

    format = PcapDump
    value = b"\xA1\xB2\xC3\xD4"
    value = b"\xD4\xC3\xB2\xA1"

    format = PcapNextGenerationDump
    value = b"\x0A\x0D\x0D\x0A"

    format = PersonalStorageTable
    value = b"!BDN"

    format = RedHatPackageManager
    value = b"\xED\xAB\xEE\xDB"

    format = Screamtracker3Module
    value = b"SCRM" offset = 44

    format = Shapefile
    value = b"\x00\x00\x27\x0A"

    format = SonyDsdStreamFile
    value = b"DSD "

    format = TagImageFileFormat
    value = b"\x4D\x4D\x00\x2A"
    value = b"\x49\x49\x2A\x00"
    value = b"\x4D\x4D\x00\x2B"
    value = b"\x49\x49\x2B\x00"

    format = Tasty
    value = b"\x5C\xA1\xAB\x1F"

    format = Wavpack
    value = b"wvpk"

    format = WebOpenFontFormat
    value = b"wOFF"

    format = WebOpenFontFormat2
    value = b"wOF2"

    format = WebassemblyBinary
    value = b"\x00\x61\x73\x6D"

    format = WindowsCursor
    value = b"\x00\x00\x02\x00"

    format = WindowsIcon
    value = b"\x00\x00\x01\x00"

    format = WindowsMetafile
    value = b"\xD7\xCD\xC6\x9A"
    value = b"\x02\x00\x09\x00"
    value = b"\x01\x00\x09\x00"

    format = Zip
    value = b"\x50\x4B\x03\x04"

    format = Zstandard
    value = b"\x28\xB5\x2F\xFD"

    // 3 bytes
    format = Bzip2
    value = b"BZh"

    format = JointPhotographicExpertsGroup
    value = b"\xFF\xD8\xFF"

    format = JpegExtendedRange
    value = b"\x49\x49\xBC"

    format = Mpeg12AudioLayer3
    value = b"ID3"
    value = b"\xFF\xE3"
    value = b"\xFF\xF3"
    value = b"\xFF\xFB"

    format = Seqbox
    value = b"SBx"

    format = SmallWebFormat
    value = b"\x43\x57\x53"
    value = b"\x46\x57\x53"

    format = Zoo
    value = b"ZOO"

    // 2 bytes
    format = AdvancedAudioCoding
    value = b"\xFF\xF1"
    value = b"\xFF\xF9"

    format = AppleDiskImage
    value = b"\x78\x01"

    format = ArchivedByRobertJung
    value = b"\x60\xEA"

    format = AudioCodec3
    value = b"\x0B\x77"

    format = CommonObjectFileFormat
    value = b"\x4C\x01"
    value = b"\x64\x86"
    value = b"\x00\x02"

    format = Cpio
    value = b"\xC7\x71"
    value = b"\x71\xC7"

    format = DerCertificate
    value = b"\x30\x82"

    format = Gzip
    value = b"\x1F\x8B"

    format = LlvmBitcode
    value = b"BC"

    format = Mpeg1AudioLayer1
    value = b"\xFF\xFE"
    value = b"\xFF\xF6"
    value = b"\xFF\xFF"

    format = Mpeg1AudioLayer2
    value = b"\xFF\xF4"
    value = b"\xFF\xF5"
    value = b"\xFF\xFC"
    value = b"\xFF\xFD"

    format = Mpeg2TransportStream
    value = b"\x47", b"\x47" offset = 188
    value = b"\x47" offset = 4, b"\x47" offset = 196

    format = MsDosExecutable
    value = b"MZ"

    format = Postscript
    value = b"%!"

    format = UnixCompress
    value = b"\x1F\xA0"
    value = b"\x1F\x9D"

    format = WindowsBitmap
    value = b"BM"
}
