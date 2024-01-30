//! Definition of known file formats signatures, listed in descending order of size.

signatures! {
    // 59 bytes
    format = MicrosoftVisualStudioSolution
    value = b"\xEF\xBB\xBF\r\nMicrosoft Visual Studio Solution File, Format Version "
    value = b"\xEF\xBB\xBF\nMicrosoft Visual Studio Solution File, Format Version "
    value = b"Microsoft Visual Studio Solution File, Format Version "

    // 52 bytes
    format = Abiword
    value = b"\xEF\xBB\xBF<!DOCTYPE abiword PUBLIC", b"<abiword template=\"false\"" offset = 102
    value = b"<!DOCTYPE abiword PUBLIC", b"<abiword template=\"false\"" offset = 102
    value = b"\xEF\xBB\xBF<abiword template=\"false\""
    value = b"<abiword template=\"false\""

    // 51 bytes
    format = AbiwordTemplate
    value = b"\xEF\xBB\xBF<!DOCTYPE abiword PUBLIC", b"<abiword template=\"true\"" offset = 102
    value = b"<!DOCTYPE abiword PUBLIC", b"<abiword template=\"true\"" offset = 102
    value = b"\xEF\xBB\xBF<abiword template=\"true\""
    value = b"<abiword template=\"true\""

    // 48 bytes
    format = JsonFeed
    value = b"{\r\n    \"version\": \"https://jsonfeed.org/version/"
    value = b"{\n    \"version\": \"https://jsonfeed.org/version/"
    value = b"{\r    \"version\": \"https://jsonfeed.org/version/"
    value = b"{\r\n  \"version\": \"https://jsonfeed.org/version/"
    value = b"{\r\n\t\"version\": \"https://jsonfeed.org/version/"
    value = b"{\n  \"version\": \"https://jsonfeed.org/version/"
    value = b"{\n\t\"version\": \"https://jsonfeed.org/version/"
    value = b"{\r  \"version\": \"https://jsonfeed.org/version/"
    value = b"{\r\t\"version\": \"https://jsonfeed.org/version/"
    value = b"{\"version\":\"https://jsonfeed.org/version/"

    // 40 bytes
    format = TimedTextMarkupLanguage
    value = b"\xEF\xBB\xBF<tt xmlns=\"http://www.w3.org/ns/ttml\""
    value = b"<tt xmlns=\"http://www.w3.org/ns/ttml\""

    // 39 bytes
    format = VirtualboxVirtualDiskImage
    value = b"<<< Oracle VM VirtualBox Disk Image >>>"

    // 37 bytes
    format = PemPrivateKey
    value = b"-----BEGIN ENCRYPTED PRIVATE KEY-----"
    value = b"-----BEGIN ECDSA PRIVATE KEY-----"
    value = b"-----BEGIN DSA PRIVATE KEY-----"
    value = b"-----BEGIN RSA PRIVATE KEY-----"
    value = b"-----BEGIN EC PRIVATE KEY-----"
    value = b"-----BEGIN PRIVATE KEY-----"

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

    // 32 bytes
    format = MicrosoftReader
    value = b"ITOLITLS\x01\0\0\0(\0\0\0", b"\xC1\x07\x90\x0A\x76\x40\xD3\x11\x87\x89\x00\x00\xF8\x10\x57\x54" offset = 24

    format = PemPublicKey
    value = b"-----BEGIN ECDSA PUBLIC KEY-----"
    value = b"-----BEGIN DSA PUBLIC KEY-----"
    value = b"-----BEGIN RSA PUBLIC KEY-----"
    value = b"-----BEGIN EC PUBLIC KEY-----"
    value = b"-----BEGIN PUBLIC KEY-----"

    format = PolygonBinary
    value = b"ply\r\nformat binary_little_endian"
    value = b"ply\nformat binary_little_endian"
    value = b"ply\rformat binary_little_endian"
    value = b"ply\r\nformat binary_big_endian"
    value = b"ply\nformat binary_big_endian"
    value = b"ply\rformat binary_big_endian"

    format = Sketchup
    value = b"\xFF\xFE\xFF\x0ES\0k\0e\0t\0c\0h\0U\0p\0 \0M\0o\0d\0e\0l\0"

    // 30 bytes
    format = FlexibleImageTransportSystem
    value =b"SIMPLE  =                    T"

    // 29 bytes
    format = NeoGeoPocketColorRom
    value = b" LICENSED BY SNK CORPORATION", b"\x10" offset = 35
    value = b"COPYRIGHT BY SNK CORPORATION", b"\x10" offset = 35

    format = PgpSignature
    value = b"-----BEGIN PGP SIGNATURE-----"

    // 28 bytes
    format = NeoGeoPocketRom
    value = b" LICENSED BY SNK CORPORATION"
    value = b"COPYRIGHT BY SNK CORPORATION"

    format = Vcalendar
    value = b"BEGIN:VCALENDAR\r\nVERSION:1.0"
    value = b"BEGIN:VCALENDAR\nVERSION:1.0"
    value = b"BEGIN:VCALENDAR\rVERSION:1.0"

    // 27 bytes
    format = PemCertificate
    value = b"-----BEGIN CERTIFICATE-----"

    format = PgpMessage
    value = b"-----BEGIN PGP MESSAGE-----"

    // 26 bytes
    format = TrainingCenterXml
    value = b"\xEF\xBB\xBF<TrainingCenterDatabase"
    value = b"<TrainingCenterDatabase"

    // 24 bytes
    format = ClojureScript
    value = b"#!/usr/local/bin/clojure"
    value = b"#!/usr/bin/env clojure"
    value = b"#!/usr/local/bin/clj"
    value = b"#!/usr/bin/clojure"
    value = b"#!/usr/bin/env clj"
    value = b"#!/usr/bin/clj"

    format = MathematicalMarkupLanguage
    value = b"\xEF\xBB\xBF<!DOCTYPE math PUBLIC"
    value = b"<!DOCTYPE math PUBLIC"
    value = b"\xEF\xBB\xBF<math"
    value = b"<math"

    // 23 bytes
    format = Extensible3d
    value = b"\xEF\xBB\xBF<!DOCTYPE X3D PUBLIC"
    value = b"<!DOCTYPE X3D PUBLIC"
    value = b"\xEF\xBB\xBF<X3D"
    value = b"\xEF\xBB\xBF<x3d"
    value = b"<X3D"
    value = b"<x3d"

    format = Opennurbs
    value = b"3D Geometry File Format"

    format = PythonScript
    value = b"#!/usr/local/bin/python"
    value = b"#!/usr/bin/env python"
    value = b"#!/usr/bin/python"

    // 22 bytes
    format = CreativeVoice
    value = b"Creative Voice File\x1A\x1A\0"

    format = ToolCommandLanguageScript
    value = b"#!/usr/local/bin/tclsh"
    value = b"#!/usr/local/bin/wish"
    value = b"#!/usr/bin/env tclsh"
    value = b"#!/usr/local/bin/tcl"
    value = b"#!/usr/bin/env wish"
    value = b"#!/usr/bin/env tcl"
    value = b"#!/usr/bin/tclsh"
    value = b"#!/usr/bin/wish"
    value = b"#!/usr/bin/tcl"

    // 21 bytes
    format = DebianBinaryPackage
    value = b"!<arch>\ndebian-binary"

    format = Filmbox
    value = b"Kaydara FBX Binary  \0"

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
    value = b"#!/bin/ash"
    value = b"#!/bin/csh"
    value = b"#!/bin/ksh"
    value = b"#!/bin/zsh"
    value = b"#!/bin/sh"

    format = WindowsMediaPlaylist
    value = b"<?wpl version=\"1.0\"?>"

    // 20 bytes
    format = InterQuakeExport
    value = b"# Inter-Quake Export"

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

    // 18 bytes
    format = DrawingExchangeFormatBinary
    value = b"AutoCAD Binary DXF"

    format = Musicxml
    value = b"\xEF\xBB\xBF<score-partwise"
    value = b"<score-partwise"

    // 17 bytes
    format = AutodeskAlias
    value = b"\x8FStudioPacketFile"

    format = EncapsulatedPostscript
    value = b"%!PS-Adobe-", b" EPSF-" offset = 14
    value = b"\xC5\xD0\xD3\xC6"

    format = HypertextMarkupLanguage
    value = b"\xEF\xBB\xBF<!DOCTYPE HTML"
    value = b"\xEF\xBB\xBF<!DOCTYPE html"
    value = b"\xEF\xBB\xBF<!doctype HTML"
    value = b"\xEF\xBB\xBF<!doctype html"
    value = b"<!DOCTYPE HTML"
    value = b"<!DOCTYPE html"
    value = b"<!doctype HTML"
    value = b"<!doctype html"

    format = PolygonAscii
    value = b"ply\r\nformat ascii"
    value = b"ply\nformat ascii"
    value = b"ply\rformat ascii"

    // 16 bytes
    format = AdobeIndesignDocument
    value = b"\x06\x06\xED\xF5\xD8\x1D\x46\xE5\xBD\x31\xEF\xE7\xFE\x74\xB7\x1D"

    format = AdvancedSystemsFormat
    value = b"\x30\x26\xB2\x75\x8E\x66\xCF\x11\xA6\xD9\x00\xAA\x00\x62\xCE\x6C"

    format = Fasttracker2ExtendedModule
    value = b"Extended Module:"

    format = InterQuakeModel
    value = b"INTERQUAKEMODEL\0"

    format = MacosAlias
    value = b"book\0\0\0\0mark\0\0\0\0"

    format = Sqlite3
    value = b"SQLite format 3\0"

    format = Stuffit
    value = b"StuffIt (c)1997", b"\x05" offset = 82
    value = b"SIT!" offset = 128, b"rLau" offset = 138
    value = b"SIT!", b"rLau" offset = 10

    format = UniversalSubtitleFormat
    value = b"\xEF\xBB\xBF<USFSubtitles"
    value = b"<USFSubtitles"

    format = VirtualRealityModelingLanguage
    value = b"#VRML V1.0 ascii"
    value = b"#VRML V2.0 utf8"

    format = WindowsRecordedTvShow
    value = b"\xB7\xD8\x00\x20\x37\x49\xDA\x11\xA6\x4E\x00\x07\xE9\x5E\xAD\x8D"

    // 15 bytes
    format = Fictionbook
    value = b"\xEF\xBB\xBF<FictionBook"
    value = b"<FictionBook"

    format = FujifilmRaw
    value = b"FUJIFILMCCD-RAW"

    format = Icalendar
    value = b"BEGIN:VCALENDAR"

    format = MegaDriveRom
    value = b"SEGA MEGA DRIVE" offset = 256
    value = b"SEGA GENESIS" offset = 256

    format = MicrosoftAccess2007Database
    value = b"Standard ACE DB" offset = 4

    format = MicrosoftAccessDatabase
    value = b"Standard Jet DB" offset = 4

    // 14 bytes
    format = CanonRaw
    value = b"II\x1A\0\0\0HEAPCCDR"

    format = Latex
    value = b"\\documentclass"
    value = b"\\documentstyle"

    format = MagickImageFileFormat
    value = b"id=ImageMagick"

    format = MaterialExchangeFormat
    value = b"\x06\x0E\x2B\x34\x02\x05\x01\x01\x0D\x01\x02\x01\x01\x02"

    format = WordperfectGraphics
    value = b"\xFFWPC\x10\0\0\0\x01\x16\x01\0", b"\0\0" offset = 14

    // 13 bytes
    format = Commodore64Cartridge
    value = b"C64 CARTRIDGE"

    format = StandardForTheExchangeOfProductModelData
    value = b"ISO-10303-21;"

    // 12 bytes
    format = Activemime
    value = b"ActiveMime\0\0"

    format = AnimatedPortableNetworkGraphics
    value = b"\x89PNG\r\n\x1A\n", b"acTL" offset = 37

    format = Appimage
    value = b"\x7FELF", b"AI\x02\x00\x00\x00\x00\x00" offset = 8

    format = CdAudio
    value = b"RIFF", b"CDDAfmt " offset = 8

    format = Djvu
    value = b"AT&TFORM", b"DJVI" offset = 12
    value = b"AT&TFORM", b"DJVM" offset = 12
    value = b"AT&TFORM", b"DJVU" offset = 12
    value = b"AT&TFORM", b"THUM" offset = 12

    format = DrawingExchangeFormatAscii
    value = b"  0\r\nSECTION"
    value = b"  0\nSECTION"

    format = JpegXl
    value = b"\0\0\0\x0CJXL \r\n\x87\n"
    value = b"\xFF\n"

    format = KhronosTexture
    value = b"\xABKTX 11\xBB\r\n\x1A\n"

    format = KhronosTexture2
    value = b"\xABKTX 20\xBB\r\n\x1A\n"

    format = MayaAscii
    value = b"//Maya ASCII"

    format = OggOpus
    value = b"OggS", b"OpusHead" offset = 28

    format = PanasonicRaw
    value = b"\x49\x49\x55\x00\x18\x00\x00\x00\x88\xE7\x74\xD8"

    format = ShoutcastPlaylist
    value = b"[playlist]\r\n"
    value = b"[playlist]\n"
    value = b"[playlist]\r"

    format = XmlShareablePlaylistFormat
    value = b"\xEF\xBB\xBF<playlist"
    value = b"<playlist"

    // 11 bytes
    format = BittorrentFile
    value = b"d8:announce"

    format = DigitalAssetExchange
    value = b"\xEF\xBB\xBF<COLLADA"
    value = b"\xEF\xBB\xBF<collada"
    value = b"<COLLADA"
    value = b"<collada"

    format = OggSpeex
    value = b"OggS", b"Speex  " offset = 28

    format = OggTheora
    value = b"OggS", b"\x80theora" offset = 28

    format = OggVorbis
    value = b"OggS", b"\x01vorbis" offset = 28

    format = RadianceHdr
    value = b"#?RADIANCE\n"

    format = TiledTilesetXml
    value = b"\xEF\xBB\xBF<tileset"
    value = b"<tileset"

    format = Vcard
    value = b"BEGIN:VCARD"

    // 10 bytes
    format = Atari7800Rom
    value = b"\x01ATARI7800"

    format = BitmapFontAscii
    value = b"info face="

    format = Drawio
    value = b"\xEF\xBB\xBF<mxfile"
    value = b"<mxfile"

    format = InitialGraphicsExchangeSpecification
    value = b"S      1\r\n" offset = 72
    value = b"S0000001\r\n" offset = 72
    value = b"S      1\n" offset = 72
    value = b"S      1\r" offset = 72
    value = b"S0000001\n" offset = 72
    value = b"S0000001\r" offset = 72

    format = MicrosoftWorksDatabase
    value = b"\x20\x54\x02\x00\x00\x00\x05\x54\x02\x00"

    format = MicrosoftWorksSpreadsheet
    value = b"\x00\x00\x02\x00\x04\x04\x05\x54\x02\x00"
    value = b"\xFF\x00\x02\x00\x04\x04\x05\x54\x02\x00"

    format = OggMedia
    value = b"OggS", b"\x01video" offset = 28

    format = Snappy
    value = b"\xFF\x06\0\0sNaPpY"

    // 9 bytes
    format = GameBoyColorRom
    value = b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B" offset = 260, b"\x80" offset = 323
    value = b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B" offset = 260, b"\xC0" offset = 323

    format = GameGearRom
    value = b"TMR SEGA" offset = 32752, b"\x50" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x51" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x5C" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x5E" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x5F" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x60" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x61" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x6C" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x6E" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x6F" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x70" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x71" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x7C" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x7E" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x7F" offset = 32767

    format = Lzop
    value = b"\x89LZO\0\r\n\x1A\n"

    format = MicrosoftVirtualHardDisk
    value = b"connectix"

    format = MsDosBatch
    value = b"@ECHO OFF"
    value = b"@echo off"

    format = OggFlac
    value = b"OggS", b"\x7FFLAC" offset = 28

    format = OlympusRawFormat
    value = b"\x49\x49\x52\x4F\x08\x00\x00\x00\x18"

    format = SegaMasterSystemRom
    value = b"TMR SEGA" offset = 32752, b"\x30" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x31" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x3C" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x3E" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x3F" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x40" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x41" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x4C" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x4E" offset = 32767
    value = b"TMR SEGA" offset = 32752, b"\x4F" offset = 32767

    format = SubripText
    value = b"\xEF\xBB\xBF1\r\n00:"
    value = b"\xEF\xBB\xBF1\n00:"
    value = b"1\r\n00:"
    value = b"1\n00:"

    format = UniversalSceneDescriptionAscii
    value = b"#usda 1.0"

    format = WebVideoTextTracks
    value = b"\xEF\xBB\xBFWEBVTT"
    value = b"WEBVTT"

    format = WindowsBitmap
    value = b"BM", b"\0\0" offset = 12, b"\0\0\0" offset = 15, b"\x01\x00" offset = 26

    format = XPixmap
    value = b"/* XPM */"

    format = XmlLocalizationInterchangeFileFormat
    value = b"\xEF\xBB\xBF<xliff"
    value = b"<xliff"

    // 8 bytes
    format = ActionsMediaVideo
    value = b"RIFF", b"AMV " offset = 8

    format = Atom
    value = b"\xEF\xBB\xBF<feed"
    value = b"<feed"

    format = AudioInterchangeFileFormat
    value = b"FORM", b"AIFC" offset = 8
    value = b"FORM", b"AIFF" offset = 8

    format = AudioVideoInterleave
    value = b"RIFF", b"AVI " offset = 8

    format = Av1ImageFileFormat
    value = b"ftypavif" offset = 4

    format = Av1ImageFileFormatSequence
    value = b"ftypavis" offset = 4

    format = BroadBandEbook
    value = b"L\0R\0F\0\0\0"

    format = CompoundFileBinary
    value = b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1"

    format = ExperimentalComputingFacility
    value = b"gimp xcf"

    format = ExtensibleMarkupLanguage
    value = b"\xEF\xBB\xBF<?xml"
    value = b"<?xml"

    format = Farbfeld
    value = b"farbfeld"

    format = GameBoyAdvanceRom
    value = b"\x24\xFF\xAE\x51\x69\x9A\xA2\x21" offset = 4

    format = GameBoyRom
    value = b"\xCE\xED\x66\x66\xCC\x0D\x00\x0B" offset = 260

    format = HighEfficiencyImageCoding
    value = b"ftypheic" offset = 4
    value = b"ftypheix" offset = 4

    format = HighEfficiencyImageCodingSequence
    value = b"ftyphevc" offset = 4
    value = b"ftyphevx" offset = 4

    format = HighEfficiencyImageFileFormat
    value = b"ftypavic" offset = 4
    value = b"ftypheim" offset = 4
    value = b"ftypheis" offset = 4
    value = b"ftypmif1" offset = 4

    format = HighEfficiencyImageFileFormatSequence
    value = b"ftypavcs" offset = 4
    value = b"ftyphevm" offset = 4
    value = b"ftyphevs" offset = 4
    value = b"ftypmsf1" offset = 4

    format = Iff8BitSampledVoice
    value = b"FORM", b"8SVX" offset = 8

    format = Jpeg2000Part1
    value = b"ftypJP2 " offset = 16
    value = b"ftypjp2 " offset = 16

    format = Jpeg2000Part2
    value = b"ftypjpx " offset = 16

    format = Jpeg2000Part3
    value = b"ftypmjp2" offset = 16

    format = Jpeg2000Part6
    value = b"ftypjpm " offset = 16

    format = JpegNetworkGraphics
    value = b"\x8BJNG\r\n\x1A\n"

    format = MayaBinary
    value = b"FOR4", b"MAYA" offset = 8
    value = b"FOR4", b"Maya" offset = 8
    value = b"FOR8", b"MAYA" offset = 16
    value = b"FOR8", b"Maya" offset = 16

    format = MicrosoftVirtualHardDisk2
    value = b"vhdxfile"

    format = Mobipocket
    value = b"BOOKMOBI" offset = 60

    format = Mpeg4Part14
    value = b"ftypARRI" offset = 4
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
    value = b"ftypNDXS" offset = 4
    value = b"ftypXAVC" offset = 4
    value = b"ftypavc1" offset = 4
    value = b"ftypdash" offset = 4
    value = b"ftypiso2" offset = 4
    value = b"ftypiso3" offset = 4
    value = b"ftypiso4" offset = 4
    value = b"ftypiso5" offset = 4
    value = b"ftypiso6" offset = 4
    value = b"ftypisom" offset = 4
    value = b"ftypmmp4" offset = 4
    value = b"ftypmobi" offset = 4
    value = b"ftypmp21" offset = 4
    value = b"ftypmp41" offset = 4
    value = b"ftypmp42" offset = 4
    value = b"ftypmp4v" offset = 4
    value = b"ftypmp71" offset = 4

    format = MultipleImageNetworkGraphics
    value = b"\x8AMNG\r\n\x1A\n"

    format = NikonElectronicFile
    value = b"\x49\x49\x2A\x00", b"\x1C\x00\xFE\x00" offset = 8
    value = b"\x49\x49\x2A\x00", b"\x1F\x00\x0B\x00" offset = 8
    value = b"\x4D\x4D\x00\x2A", b"\x1C\x00\xFE\x00" offset = 8
    value = b"\x4D\x4D\x00\x2A", b"\x1F\x00\x0B\x00" offset = 8

    format = Nintendo64Rom
    value = b"\x12\x40\x80\x37\x00\x0F\x00\x00"
    value = b"\x37\x80\x40\x12\x00\x00\x0F\x00"
    value = b"\x40\x12\x37\x80\x0F\x00\x00\x00"
    value = b"\x80\x37\x12\x40\x00\x00\x00\x0F"

    format = NintendoDsRom
    value = b"\x24\xFF\xAE\x51\x69\x9A\xA2\x21" offset = 192
    value = b"\xC8\x60\x4F\xE2\x01\x70\x8F\xE2" offset = 192

    format = PortableNetworkGraphics
    value = b"\x89PNG\r\n\x1A\n"

    format = QualcommPurevoice
    value = b"RIFF", b"QLCM" offset = 8

    format = Realmedia
    value = b".RMF\0\0\0\x12"

    format = RoshalArchive
    value = b"Rar!\x1A\x07\x01\0"
    value = b"Rar!\x1A\x07\0"

    format = SimpleObjectAccessProtocol
    value = b"\xEF\xBB\xBF<soap"
    value = b"<soap"

    format = Soundfont2
    value = b"RIFF", b"sfbk" offset = 8

    format = StuffitX
    value = b"StuffIt!"
    value = b"StuffIt?"

    format = TapeArchive
    value = b"ustar\0\x30\x30" offset = 257
    value = b"ustar  \0" offset = 257

    format = ThirdGenerationPartnershipProject2
    value = b"ftypKDDI" offset = 4
    value = b"ftyp3g2" offset = 4

    format = UniversalSceneDescriptionBinary
    value = b"PXR-USDC"

    format = WaveformAudio
    value = b"RIFF", b"WAVE" offset = 8

    format = Webp
    value = b"RIFF", b"WEBP" offset = 8

    format = WindowsAnimatedCursor
    value = b"RIFF", b"ACON" offset = 8

    format = WindowsImagingFormat
    value = b"MSWIM\0\0\0"
    value = b"WLPWM\0\0\0"

    // 7 bytes
    format = AdditiveManufacturingFormat
    value = b"\xEF\xBB\xBF<amf"
    value = b"<amf"

    format = AdvancedCompressionEngine
    value = b"**ACE**" offset = 7

    format = AdvancedStreamRedirector
    value = b"\xEF\xBB\xBF<ASX"
    value = b"\xEF\xBB\xBF<asx"
    value = b"<ASX"
    value = b"<asx"

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

    format = Cinema4d
    value = b"C4DC4D6" offset = 1

    format = ExtensibleStylesheetLanguageTransformations
    value = b"\xEF\xBB\xBF<xsl"
    value = b"<xsl"

    format = FlashMp4Audio
    value = b"ftypF4A" offset = 4
    value = b"ftypf4a" offset = 4

    format = FlashMp4Audiobook
    value = b"ftypF4B" offset = 4
    value = b"ftypf4b" offset = 4

    format = FlashMp4ProtectedVideo
    value = b"ftypF4P" offset = 4
    value = b"ftypf4p" offset = 4

    format = FlashMp4Video
    value = b"ftypF4V" offset = 4
    value = b"ftypf4v" offset = 4

    format = GeographyMarkupLanguage
    value = b"\xEF\xBB\xBF<gml"
    value = b"<gml"

    format = GpsExchangeFormat
    value = b"\xEF\xBB\xBF<gpx"
    value = b"<gpx"

    format = KeyholeMarkupLanguage
    value = b"\xEF\xBB\xBF<kml"
    value = b"<kml"

    format = MetaInformationEncapsulation
    value = b"\x7E\x10\x04", b"0MIE" offset = 4
    value = b"\x7E\x18\x04", b"0MIE" offset = 4

    format = Model3dAscii
    value = b"3dmodel"

    format = Mp3Url
    value = b"#EXTM3U"

    format = MpegDashManifest
    value = b"\xEF\xBB\xBF<MPD"
    value = b"<MPD"

    format = MultiLayerArchive
    value = b"MLA\x01\x00\x00\x00"

    format = ReallySimpleSyndication
    value = b"\xEF\xBB\xBF<rss"
    value = b"<rss"

    format = ScalableVectorGraphics
    value = b"\xEF\xBB\xBF<SVG"
    value = b"\xEF\xBB\xBF<svg"
    value = b"<SVG"
    value = b"<svg"

    format = SonyMovie
    value = b"ftypmqt" offset = 4

    format = ThirdGenerationPartnershipProject
    value = b"ftyp3ge" offset = 4
    value = b"ftyp3gf" offset = 4
    value = b"ftyp3gg" offset = 4
    value = b"ftyp3gh" offset = 4
    value = b"ftyp3gm" offset = 4
    value = b"ftyp3gp" offset = 4
    value = b"ftyp3gr" offset = 4
    value = b"ftyp3gs" offset = 4
    value = b"ftyp3gt" offset = 4

    format = TiledMapXml
    value = b"\xEF\xBB\xBF<map"
    value = b"<map"

    format = UnixArchiver
    value = b"!<arch>"

    format = WebassemblyText
    value = b"(module"

    // 6 bytes
    format = ApacheArrowColumnar
    value = b"ARROW1"

    format = AppleQuicktime
    value = b"ftypqt" offset = 4
    value = b"free" offset = 4
    value = b"mdat" offset = 4
    value = b"moov" offset = 4
    value = b"skip" offset = 4
    value = b"wide" offset = 4

    format = CanonRaw2
    value = b"\x49\x49\x2A\x00", b"CR" offset = 8
    value = b"\x4D\x4D\x00\x2A", b"CR" offset = 8

    format = DesignWebFormat
    value = b"(DWF V"

    format = GraphicsInterchangeFormat
    value = b"GIF87a"
    value = b"GIF89a"

    format = MicrosoftWrite
    value = b"\x31\xBE\x00\x00\x00\xAB"
    value = b"\x32\xBE\x00\x00\x00\xAB"

    format = SevenZip
    value = b"7z\xBC\xAF\x27\x1C"

    format = StereolithographyAscii
    value = b"solid "

    format = ThreeDimensionalStudio
    value = b"MM", b"\x02" offset = 6, b"\n" offset = 8, b"==" offset = 16

    format = WordperfectMacro
    value = b"\xFFWPC", b"\x01\x01" offset = 8

    format = Xz
    value = b"\xFD7zXZ\0"

    // 5 bytes
    format = AdaptiveMultiRate
    value = b"#!AMR"

    format = Bzip3
    value = b"BZ3v1"

    format = CorelPresentations
    value = b"\xFFWPC", b"\x0F" offset = 9
    value = b"\xFFWPC", b"\x10" offset = 9

    format = DalvikExecutable
    value = b"dex\n", b"\0" offset = 7

    format = EmbeddedOpentype
    value = b"\x00\x00\x01" offset = 8, b"LP" offset = 34
    value = b"\x01\x00\x02" offset = 8, b"LP" offset = 34
    value = b"\x02\x00\x02" offset = 8, b"LP" offset = 34

    format = GoogleDraco
    value = b"DRACO"

    format = Iso9660
    value = b"CD001" offset = 32769
    value = b"CD001" offset = 34817
    value = b"CD001" offset = 36865

    format = Larc
    value = b"-lz2-" offset = 2
    value = b"-lz3-" offset = 2
    value = b"-lz4-" offset = 2
    value = b"-lz5-" offset = 2
    value = b"-lz7-" offset = 2
    value = b"-lz8-" offset = 2
    value = b"-lzs-" offset = 2

    format = LempelZivMarkovChainAlgorithm
    value = b"\x5D\x00\x00\x80\x00"

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
    value = b"\xCE\xFA\xED\xFE"
    value = b"\xCF\xFA\xED\xFE"
    value = b"\xFE\xED\xFA\xCE"
    value = b"\xFE\xED\xFA\xCF"

    format = NintendoSwitchRom
    value = b"HEAD" offset = 256, b"\xE0" offset = 269
    value = b"HEAD" offset = 256, b"\xE1" offset = 269
    value = b"HEAD" offset = 256, b"\xE2" offset = 269
    value = b"HEAD" offset = 256, b"\xF0" offset = 269
    value = b"HEAD" offset = 256, b"\xF8" offset = 269
    value = b"HEAD" offset = 256, b"\xFA" offset = 269

    format = Opentype
    value = b"OTTO\0"

    format = Pmarc
    value = b"-pc1-" offset = 2
    value = b"-pm0-" offset = 2
    value = b"-pm1-" offset = 2
    value = b"-pm2-" offset = 2
    value = b"-pms-" offset = 2

    format = PortableDocumentFormat
    value = b"%PDF-"

    format = RichTextFormat
    value = b"{\\rtf"

    format = Truetype
    value = b"\x00\x01\x00\x00\x00"

    format = VirtualMachineDisk
    value = b"COWD\x01"
    value = b"KDMV\x01"
    value = b"KDMV\x02"
    value = b"KDMV\x03"

    format = WordperfectPresentations
    value = b"\xFFWPC", b"\x0A" offset = 9

    // 4 bytes
    format = AdaptableScalableTextureCompression
    value = b"\x13\xAB\xA1\x5C"

    format = AdobePhotoshopDocument
    value = b"8BPS"

    format = Alz
    value = b"ALZ\x01"

    format = AmigaDiskFile
    value = b"DOS\0"
    value = b"DOS\x01"
    value = b"DOS\x02"
    value = b"DOS\x03"
    value = b"DOS\x04"
    value = b"DOS\x05"

    format = AndroidBinaryXml
    value = b"\x03\x00\x08\x00"

    format = AndroidCompiledResources
    value = b"\x02\x00\x0C\x00"

    format = ApacheAvroObjectContainer
    value = b"Obj\x01"

    format = ApacheParquet
    value = b"PAR1"

    format = AppleIconImage
    value = b"icns"

    format = Au
    value = b".snd"

    format = AudioVisualResearch
    value = b"2BIT"

    format = AutocadDrawing
    value = b"AC10"

    format = BetterPortableGraphics
    value = b"BPG\xFB"

    format = BitmapFontBinary
    value = b"BMF\x03"

    format = Cabinet
    value = b"ISc("
    value = b"MSCF"

    format = Cineon
    value = b"\x80\x2A\x5F\xD7"

    format = Cpio
    value = b"0707"
    value = b"\x71\xC7"
    value = b"\xC7\x71"

    format = DigitalImagingAndCommunicationsInMedicine
    value = b"DICM" offset = 128

    format = DigitalPictureExchange
    value = b"SDPX"
    value = b"XPDS"

    format = ExecutableAndLinkableFormat
    value = b"\x7FELF"

    format = ExtensibleArchive
    value = b"xar!"

    format = ExtensibleBinaryMetaLanguage
    value = b"\x1A\x45\xDF\xA3"

    format = FlashVideo
    value = b"FLV\x01"

    format = FlexibleAndInteroperableDataTransfer
    value = b".FIT" offset = 8

    format = FreeLosslessAudioCodec
    value = b"fLaC"

    format = FreeLosslessImageFormat
    value = b"FLIF"

    format = GettextMachineObject
    value = b"\x95\x04\x12\xDE"
    value = b"\xDE\x12\x04\x95"

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

    format = Jpeg2000Codestream
    value = b"\xFF\x4F\xFF\x51"

    format = JpegLs
    value = b"\xFF\xD8\xFF\xF7"

    format = LempelZivFiniteStateEntropy
    value = b"bvx-"
    value = b"bvx1"
    value = b"bvx2"
    value = b"bvxn"

    format = LlvmBitcode
    value = b"BC\xC0\xDE"

    format = LongRangeZip
    value = b"LRZI"

    format = LuaBytecode
    value = b"\x1BLua"

    format = Lz4
    value = b"\x04\x22\x4D\x18"

    format = Lzip
    value = b"LZIP"

    format = Magicavoxel
    value = b"VOX "

    format = MicrosoftCompiledHtmlHelp
    value = b"ITSF"

    format = MicrosoftDirectdrawSurface
    value = b"DDS "

    format = MicrosoftWorksWordProcessor
    value = b"\x01\xFE", b"\x01\x00" offset = 112

    format = Model3dBinary
    value = b"3DMO"

    format = MonkeysAudio
    value = b"MAC "

    format = MozillaArchive
    value = b"MAR1"

    format = Mpeg12Video
    value = b"\x00\x00\x01\xB3"
    value = b"\x00\x00\x01\xBA"

    format = Musepack
    value = b"MPCK"
    value = b"MP+"

    format = MusicalInstrumentDigitalInterface
    value = b"MThd"

    format = NintendoEntertainmentSystemRom
    value = b"NES\x1A"

    format = NintendoSwitchExecutable
    value = b"NSO0"

    format = NintendoSwitchPackage
    value = b"PFS0"

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

    format = PictureExchange
    value = b"\x0A\x00\x00", b"\x00" offset = 64
    value = b"\x0A\x00\x01", b"\x00" offset = 64
    value = b"\x0A\x02\x00", b"\x00" offset = 64
    value = b"\x0A\x02\x01", b"\x00" offset = 64
    value = b"\x0A\x03\x00", b"\x00" offset = 64
    value = b"\x0A\x03\x01", b"\x00" offset = 64
    value = b"\x0A\x04\x00", b"\x00" offset = 64
    value = b"\x0A\x04\x01", b"\x00" offset = 64
    value = b"\x0A\x05\x00", b"\x00" offset = 64
    value = b"\x0A\x05\x01", b"\x00" offset = 64

    format = QemuCopyOnWrite
    value = b"QFI\xFB"

    format = QuiteOkAudio
    value = b"qoaf"

    format = QuiteOkImage
    value = b"qoif"

    format = Realaudio
    value = b".ra\xFD"

    format = RedHatPackageManager
    value = b"\xED\xAB\xEE\xDB"

    format = Rzip
    value = b"RZIP"

    format = ScreamTracker3Module
    value = b"SCRM" offset = 44

    format = Shapefile
    value = b"\0\0'\n"

    format = SiliconGraphicsMovie
    value = b"MOVI"

    format = SonyDsdStreamFile
    value = b"DSD "

    format = Squashfs
    value = b"hsqs"

    format = TagImageFileFormat
    value = b"\x49\x49\x2A\x00"
    value = b"\x49\x49\x2B\x00"
    value = b"\x4D\x4D\x00\x2A"
    value = b"\x4D\x4D\x00\x2B"

    format = Tasty
    value = b"\x5C\xA1\xAB\x1F"

    format = UltimateSoundtrackerModule
    value = b"M.K." offset = 1080

    format = Universal3d
    value = b"U3D\0"

    format = Wavpack
    value = b"wvpk"

    format = WebOpenFontFormat
    value = b"wOFF"

    format = WebOpenFontFormat2
    value = b"wOF2"

    format = WebassemblyBinary
    value = b"\0asm"

    format = WindowsCursor
    value = b"\x00\x00\x02\x00"

    format = WindowsIcon
    value = b"\x00\x00\x01\x00"

    format = WindowsMetafile
    value = b"\x01\x00\x09\x00"
    value = b"\x02\x00\x09\x00"
    value = b"\xD7\xCD\xC6\x9A"

    format = WordperfectDocument
    value = b"\xFFWPC"

    format = Xbox360Executable
    value = b"XEX1"
    value = b"XEX2"

    format = XboxExecutable
    value = b"XBEH"

    format = Zip
    value = b"PK\x03\x04"
    value = b"PK\x05\x06"

    format = Zpaq
    value = b"7kSt"
    value = b"zPQ"

    format = Zstandard
    value = b"\x28\xB5\x2F\xFD"

    // 3 bytes
    format = ArchivedByRobertJung
    value = b"\x60\xEA", b"\x02" offset = 10

    format = Bzip
    value = b"BZ0"

    format = Bzip2
    value = b"BZh"

    format = Commodore64Program
    value = b"\x01\x08", b"\x9E" offset = 6

    format = JointPhotographicExpertsGroup
    value = b"\xFF\xD8\xFF"

    format = JpegExtendedRange
    value = b"\x49\x49\xBC"

    format = Mpeg12AudioLayer3
    value = b"ID3"
    value = b"\xFF\xE2"
    value = b"\xFF\xE3"
    value = b"\xFF\xF2"
    value = b"\xFF\xF3"
    value = b"\xFF\xFA"
    value = b"\xFF\xFB"

    format = Mtv
    value = b"AMV"

    format = PortableArbitraryMap
    value = b"P7 "
    value = b"P7\n"
    value = b"P7\r"
    value = b"P7\t"

    format = PortableBitmap
    value = b"P1 "
    value = b"P1\n"
    value = b"P1\r"
    value = b"P1\t"
    value = b"P4 "
    value = b"P4\n"
    value = b"P4\r"
    value = b"P4\t"

    format = PortableFloatmap
    value = b"PF "
    value = b"PF\n"
    value = b"PF\r"
    value = b"PF\t"
    value = b"Pf "
    value = b"Pf\n"
    value = b"Pf\r"
    value = b"Pf\t"

    format = PortableGraymap
    value = b"P2 "
    value = b"P2\n"
    value = b"P2\r"
    value = b"P2\t"
    value = b"P5 "
    value = b"P5\n"
    value = b"P5\r"
    value = b"P5\t"

    format = PortablePixmap
    value = b"P3 "
    value = b"P3\n"
    value = b"P3\r"
    value = b"P3\t"
    value = b"P6 "
    value = b"P6\n"
    value = b"P6\r"
    value = b"P6\t"

    format = Seqbox
    value = b"SBx"

    format = SmallWebFormat
    value = b"CWS"
    value = b"FWS"
    value = b"ZWS"

    format = Zoo
    value = b"ZOO"

    // 2 bytes
    format = AdvancedAudioCoding
    value = b"\xFF\xF1"
    value = b"\xFF\xF9"

    format = AppleDiskImage
    value = b"\x78\x01"

    format = AudioCodec3
    value = b"\x0B\x77"

    format = AutodeskAnimator
    value = b"\x11\xAF" offset = 4

    format = AutodeskAnimatorPro
    value = b"\x12\xAF" offset = 4

    format = BdavMpeg2TransportStream
    value = b"\x47" offset = 4, b"\x47" offset = 196

    format = CommonObjectFileFormat
    value = b"\x00\x02"
    value = b"\x4C\x01"
    value = b"\x64\x86"

    format = DerCertificate
    value = b"\x30\x82"

    format = Gzip
    value = b"\x1F\x8B"

    format = Mpeg12AudioLayer2
    value = b"\xFF\xF4"
    value = b"\xFF\xF5"
    value = b"\xFF\xFC"
    value = b"\xFF\xFD"

    format = Mpeg2TransportStream
    value = b"\x47", b"\x47" offset = 188

    format = MsDosExecutable
    value = b"MZ"
    value = b"ZM"

    format = Postscript
    value = b"%!"

    format = SiliconGraphicsImage
    value = b"\x01\xDA"

    format = UnixCompress
    value = b"\x1F\x9D"
    value = b"\x1F\xA0"
}
