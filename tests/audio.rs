use file_format::FileFormat;

#[test]
fn test_adaptive_multi_rate() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.amr").unwrap();
    assert_eq!(fmt, FileFormat::AdaptiveMultiRate);
}

#[test]
fn test_adobe_flash_player_audio() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.f4a").unwrap();
    assert_eq!(fmt, FileFormat::AdobeFlashPlayerAudio);
}

#[test]
fn test_adobe_flash_player_audiobook() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.f4b").unwrap();
    assert_eq!(fmt, FileFormat::AdobeFlashPlayerAudiobook);
}

#[test]
fn test_advanced_audio_coding() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.aac").unwrap();
    assert_eq!(fmt, FileFormat::AdvancedAudioCoding);
}

#[test]
fn test_apple_itunes_audio() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.m4a").unwrap();
    assert_eq!(fmt, FileFormat::AppleItunesAudio);
}

#[test]
fn test_apple_itunes_audiobook() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.m4b").unwrap();
    assert_eq!(fmt, FileFormat::AppleItunesAudiobook);
}

#[test]
fn test_apple_itunes_protected_audio() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.m4p").unwrap();
    assert_eq!(fmt, FileFormat::AppleItunesProtectedAudio);
}

#[test]
fn test_au() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.au").unwrap();
    assert_eq!(fmt, FileFormat::Au);
}

#[test]
fn test_audio_codec3() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.ac3").unwrap();
    assert_eq!(fmt, FileFormat::AudioCodec3);
}

#[test]
fn test_audio_interchange_file_format() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.aiff").unwrap();
    assert_eq!(fmt, FileFormat::AudioInterchangeFileFormat);
}

#[test]
fn test_audio_visual_research() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.avr").unwrap();
    assert_eq!(fmt, FileFormat::AudioVisualResearch);
}

#[test]
fn test_creative_voice() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.voc").unwrap();
    assert_eq!(fmt, FileFormat::CreativeVoice);
}

#[test]
fn test_fasttracker2_extended_module() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.xm").unwrap();
    assert_eq!(fmt, FileFormat::Fasttracker2ExtendedModule);
}

#[test]
fn test_free_lossless_audio_codec() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.flac").unwrap();
    assert_eq!(fmt, FileFormat::FreeLosslessAudioCodec);
}

#[test]
fn test_iff_8_bit_sampled_voice() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.8svx").unwrap();
    assert_eq!(fmt, FileFormat::Iff8BitSampledVoice);
}

#[test]
fn test_impulse_tracker_module() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.it").unwrap();
    assert_eq!(fmt, FileFormat::ImpulseTrackerModule);
}

#[cfg(feature = "reader-ebml")]
#[test]
fn test_matroska_audio() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.mka").unwrap();
    assert_eq!(fmt, FileFormat::MatroskaAudio);
}

#[test]
fn test_monkeys_audio() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.ape").unwrap();
    assert_eq!(fmt, FileFormat::MonkeysAudio);
}

#[test]
fn test_mpeg12_audio_layer3() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.mp3").unwrap();
    assert_eq!(fmt, FileFormat::Mpeg12AudioLayer3);
}

#[test]
fn test_mpeg1_audio_layer1() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.mp1").unwrap();
    assert_eq!(fmt, FileFormat::Mpeg1AudioLayer1);
}

#[test]
fn test_mpeg1_audio_layer2() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.mp2").unwrap();
    assert_eq!(fmt, FileFormat::Mpeg1AudioLayer2);
}

#[test]
fn test_musepack() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.mpc").unwrap();
    assert_eq!(fmt, FileFormat::Musepack);
}

#[test]
fn test_musical_instrument_digital_interface() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.mid").unwrap();
    assert_eq!(fmt, FileFormat::MusicalInstrumentDigitalInterface);
}

#[test]
fn test_ogg_flac() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.oga").unwrap();
    assert_eq!(fmt, FileFormat::OggFlac);
}

#[test]
fn test_ogg_opus() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.opus").unwrap();
    assert_eq!(fmt, FileFormat::OggOpus);
}

#[test]
fn test_ogg_speex() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.spx").unwrap();
    assert_eq!(fmt, FileFormat::OggSpeex);
}

#[test]
fn test_ogg_vorbis() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.ogg").unwrap();
    assert_eq!(fmt, FileFormat::OggVorbis);
}

#[test]
fn test_qualcomm_purevoice() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.qcp").unwrap();
    assert_eq!(fmt, FileFormat::QualcommPurevoice);
}

#[test]
fn test_quite_ok_audio() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.qoa").unwrap();
    assert_eq!(fmt, FileFormat::QuiteOkAudio);
}

#[test]
fn test_realaudio_1() {
    let fmt = FileFormat::from_file("fixtures/audio/sample1.ra").unwrap();
    assert_eq!(fmt, FileFormat::Realaudio);
}

#[cfg(feature = "reader-rm")]
#[test]
fn test_realaudio_2() {
    let fmt = FileFormat::from_file("fixtures/audio/sample2.ra").unwrap();
    assert_eq!(fmt, FileFormat::Realaudio);
}

#[test]
fn test_scream_tracker3_module() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.s3m").unwrap();
    assert_eq!(fmt, FileFormat::ScreamTracker3Module);
}

#[test]
fn test_sony_dsd_stream_file() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.dsf").unwrap();
    assert_eq!(fmt, FileFormat::SonyDsdStreamFile);
}

#[test]
fn test_soundfont2() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.sf2").unwrap();
    assert_eq!(fmt, FileFormat::Soundfont2);
}

#[test]
fn test_ultimate_soundtracker_module() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.mod").unwrap();
    assert_eq!(fmt, FileFormat::UltimateSoundtrackerModule);
}

#[test]
fn test_waveform_audio() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.wav").unwrap();
    assert_eq!(fmt, FileFormat::WaveformAudio);
}

#[test]
fn test_wavpack() {
    let fmt = FileFormat::from_file("fixtures/audio/sample.wv").unwrap();
    assert_eq!(fmt, FileFormat::Wavpack);
}
