use file_format::FileFormat;

#[test]
fn test_adaptive_multi_rate() {
    let format = FileFormat::from_file("fixtures/audio/sample.amr").unwrap();
    assert_eq!(format, FileFormat::AdaptiveMultiRate);
}

#[test]
fn test_adobe_flash_player_audio() {
    let format = FileFormat::from_file("fixtures/audio/sample.f4a").unwrap();
    assert_eq!(format, FileFormat::AdobeFlashPlayerAudio);
}

#[test]
fn test_adobe_flash_player_audiobook() {
    let format = FileFormat::from_file("fixtures/audio/sample.f4b").unwrap();
    assert_eq!(format, FileFormat::AdobeFlashPlayerAudiobook);
}

#[test]
fn test_advanced_audio_coding() {
    let format = FileFormat::from_file("fixtures/audio/sample.aac").unwrap();
    assert_eq!(format, FileFormat::AdvancedAudioCoding);
}

#[test]
fn test_apple_itunes_audio() {
    let format = FileFormat::from_file("fixtures/audio/sample.m4a").unwrap();
    assert_eq!(format, FileFormat::AppleItunesAudio);
}

#[test]
fn test_apple_itunes_audiobook() {
    let format = FileFormat::from_file("fixtures/audio/sample.m4b").unwrap();
    assert_eq!(format, FileFormat::AppleItunesAudiobook);
}

#[test]
fn test_apple_itunes_protected_audio() {
    let format = FileFormat::from_file("fixtures/audio/sample.m4p").unwrap();
    assert_eq!(format, FileFormat::AppleItunesProtectedAudio);
}

#[test]
fn test_au() {
    let format = FileFormat::from_file("fixtures/audio/sample.au").unwrap();
    assert_eq!(format, FileFormat::Au);
}

#[test]
fn test_audio_codec3() {
    let format = FileFormat::from_file("fixtures/audio/sample.ac3").unwrap();
    assert_eq!(format, FileFormat::AudioCodec3);
}

#[test]
fn test_audio_interchange_file_format() {
    let format = FileFormat::from_file("fixtures/audio/sample.aiff").unwrap();
    assert_eq!(format, FileFormat::AudioInterchangeFileFormat);
}

#[test]
fn test_creative_voice() {
    let format = FileFormat::from_file("fixtures/audio/sample.voc").unwrap();
    assert_eq!(format, FileFormat::CreativeVoice);
}

#[test]
fn test_fasttracker2_extended_module() {
    let format = FileFormat::from_file("fixtures/audio/sample.xm").unwrap();
    assert_eq!(format, FileFormat::Fasttracker2ExtendedModule);
}

#[test]
fn test_free_lossless_audio_codec() {
    let format = FileFormat::from_file("fixtures/audio/sample.flac").unwrap();
    assert_eq!(format, FileFormat::FreeLosslessAudioCodec);
}

#[test]
fn test_impulse_tracker_module() {
    let format = FileFormat::from_file("fixtures/audio/sample.it").unwrap();
    assert_eq!(format, FileFormat::ImpulseTrackerModule);
}

#[test]
fn test_monkeys_audio() {
    let format = FileFormat::from_file("fixtures/audio/sample.ape").unwrap();
    assert_eq!(format, FileFormat::MonkeysAudio);
}

#[test]
fn test_mp3_url() {
    let format = FileFormat::from_file("fixtures/audio/sample.m3u").unwrap();
    assert_eq!(format, FileFormat::Mp3Url);
}

#[test]
fn test_mpeg12_audio_layer3() {
    let format = FileFormat::from_file("fixtures/audio/sample.mp3").unwrap();
    assert_eq!(format, FileFormat::Mpeg12AudioLayer3);
}

#[test]
fn test_mpeg1_audio_layer1() {
    let format = FileFormat::from_file("fixtures/audio/sample.mp1").unwrap();
    assert_eq!(format, FileFormat::Mpeg1AudioLayer1);
}

#[test]
fn test_mpeg1_audio_layer2() {
    let format = FileFormat::from_file("fixtures/audio/sample.mp2").unwrap();
    assert_eq!(format, FileFormat::Mpeg1AudioLayer2);
}

#[test]
fn test_musepack() {
    let format = FileFormat::from_file("fixtures/audio/sample.mpc").unwrap();
    assert_eq!(format, FileFormat::Musepack);
}

#[test]
fn test_musical_instrument_digital_interface() {
    let format = FileFormat::from_file("fixtures/audio/sample.mid").unwrap();
    assert_eq!(format, FileFormat::MusicalInstrumentDigitalInterface);
}

#[test]
fn test_ogg_flac() {
    let format = FileFormat::from_file("fixtures/audio/sample.oga").unwrap();
    assert_eq!(format, FileFormat::OggFlac);
}

#[test]
fn test_ogg_opus() {
    let format = FileFormat::from_file("fixtures/audio/sample.opus").unwrap();
    assert_eq!(format, FileFormat::OggOpus);
}

#[test]
fn test_ogg_speex() {
    let format = FileFormat::from_file("fixtures/audio/sample.spx").unwrap();
    assert_eq!(format, FileFormat::OggSpeex);
}

#[test]
fn test_ogg_vorbis() {
    let format = FileFormat::from_file("fixtures/audio/sample.ogg").unwrap();
    assert_eq!(format, FileFormat::OggVorbis);
}

#[test]
fn test_qualcomm_pure_voice() {
    let format = FileFormat::from_file("fixtures/audio/sample.qcp").unwrap();
    assert_eq!(format, FileFormat::QualcommPureVoice);
}

#[test]
fn test_quite_ok_audio() {
    let format = FileFormat::from_file("fixtures/audio/sample.qoa").unwrap();
    assert_eq!(format, FileFormat::QuiteOkAudio);
}

#[test]
fn test_scream_tracker3_module() {
    let format = FileFormat::from_file("fixtures/audio/sample.s3m").unwrap();
    assert_eq!(format, FileFormat::ScreamTracker3Module);
}

#[test]
fn test_shoutcast_playlist() {
    let format = FileFormat::from_file("fixtures/audio/sample.pls").unwrap();
    assert_eq!(format, FileFormat::ShoutcastPlaylist);
}

#[test]
fn test_sony_dsd_stream_file() {
    let format = FileFormat::from_file("fixtures/audio/sample.dsf").unwrap();
    assert_eq!(format, FileFormat::SonyDsdStreamFile);
}

#[test]
fn test_ultimate_soundtracker_module() {
    let format = FileFormat::from_file("fixtures/audio/sample.mod").unwrap();
    assert_eq!(format, FileFormat::UltimateSoundtrackerModule);
}

#[test]
fn test_waveform_audio() {
    let format = FileFormat::from_file("fixtures/audio/sample.wav").unwrap();
    assert_eq!(format, FileFormat::WaveformAudio);
}

#[test]
fn test_wavpack() {
    let format = FileFormat::from_file("fixtures/audio/sample.wv").unwrap();
    assert_eq!(format, FileFormat::Wavpack);
}
