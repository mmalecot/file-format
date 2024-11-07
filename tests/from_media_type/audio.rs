use file_format::FileFormat;

#[test]
#[cfg(feature = "from-media-type")]
fn test_adaptive_multi_rate() {
    let fmt = FileFormat::from_media_type("audio/amr");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AdaptiveMultiRate)), "{:?} does not contain {}", fmt, FileFormat::AdaptiveMultiRate);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_advanced_audio_coding() {
    let fmt = FileFormat::from_media_type("audio/aac");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AdvancedAudioCoding)), "{:?} does not contain {}", fmt, FileFormat::AdvancedAudioCoding);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_apple_itunes_audio() {
    let fmt = FileFormat::from_media_type("audio/x-m4a");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AppleItunesAudio)), "{:?} does not contain {}", fmt, FileFormat::AppleItunesAudio);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_apple_itunes_audiobook() {
    let fmt = FileFormat::from_media_type("audio/mp4");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AppleItunesAudiobook)), "{:?} does not contain {}", fmt, FileFormat::AppleItunesAudiobook);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_apple_itunes_protected_audio() {
    let fmt = FileFormat::from_media_type("audio/mp4");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AppleItunesProtectedAudio)), "{:?} does not contain {}", fmt, FileFormat::AppleItunesProtectedAudio);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_au(){
    let fmt = FileFormat::from_media_type("audio/basic");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Au)), "{:?} does not contain {}", fmt, FileFormat::Au);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_audio_codec3() {
    let fmt = FileFormat::from_media_type("audio/ac3");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AudioCodec3)), "{:?} does not contain {}", fmt, FileFormat::AudioCodec3);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_audio_interchange_file_format() {
    let fmt = FileFormat::from_media_type("audio/x-aiff");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AudioInterchangeFileFormat)), "{:?} does not contain {}", fmt, FileFormat::AudioInterchangeFileFormat);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_audio_visual_research() {
    let fmt = FileFormat::from_media_type("audio/x-avr");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AudioVisualResearch)), "{:?} does not contain {}", fmt, FileFormat::AudioVisualResearch);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_creative_voice() {
    let fmt = FileFormat::from_media_type("audio/x-voc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CreativeVoice)), "{:?} does not contain {}", fmt, FileFormat::CreativeVoice);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_eight_bit_sampled_voice() {
    let fmt = FileFormat::from_media_type("audio/x-8svx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::EightBitSampledVoice)), "{:?} does not contain {}", fmt, FileFormat::EightBitSampledVoice);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_fasttracker2_extended_module() {
    let fmt = FileFormat::from_media_type("audio/x-xm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Fasttracker2ExtendedModule)), "{:?} does not contain {}", fmt, FileFormat::Fasttracker2ExtendedModule);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_flash_mp4_audio() {
    let fmt = FileFormat::from_media_type("audio/mp4");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FlashMp4Audio)), "{:?} does not contain {}", fmt, FileFormat::FlashMp4Audio);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_flash_mp4_audiobook() {
    let fmt = FileFormat::from_media_type("audio/mp4");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FlashMp4Audiobook)), "{:?} does not contain {}", fmt, FileFormat::FlashMp4Audiobook);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_free_lossless_audio_codec() {
    let fmt = FileFormat::from_media_type("audio/x-flac");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FreeLosslessAudioCodec)), "{:?} does not contain {}", fmt, FileFormat::FreeLosslessAudioCodec);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_impulse_tracker_module() {
    let fmt = FileFormat::from_media_type("audio/x-it");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ImpulseTrackerModule)), "{:?} does not contain {}", fmt, FileFormat::ImpulseTrackerModule);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_matroska_audio() {
    let fmt = FileFormat::from_media_type("audio/x-matroska");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MatroskaAudio)), "{:?} does not contain {}", fmt, FileFormat::MatroskaAudio);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_monkeys_audio() {
    let fmt = FileFormat::from_media_type("audio/x-ape");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MonkeysAudio)), "{:?} does not contain {}", fmt, FileFormat::MonkeysAudio);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mpeg12_audio_layer2() {
    let fmt = FileFormat::from_media_type("audio/mpeg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mpeg12AudioLayer2)), "{:?} does not contain {}", fmt, FileFormat::Mpeg12AudioLayer2);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mpeg12_audio_layer3() {
    let fmt = FileFormat::from_media_type("audio/mpeg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mpeg12AudioLayer3)), "{:?} does not contain {}", fmt, FileFormat::Mpeg12AudioLayer3);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_mpeg4_part14_audio() {
    let fmt = FileFormat::from_media_type("audio/mp4");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mpeg4Part14Audio)), "{:?} does not contain {}", fmt, FileFormat::Mpeg4Part14Audio);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_musepack(){
    let fmt = FileFormat::from_media_type("audio/x-musepack");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Musepack)), "{:?} does not contain {}", fmt, FileFormat::Musepack);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_musical_instrument_digital_interface() {
    let fmt = FileFormat::from_media_type("audio/midi");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MusicalInstrumentDigitalInterface)), "{:?} does not contain {}", fmt, FileFormat::MusicalInstrumentDigitalInterface);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_ogg_flac() {
    let fmt = FileFormat::from_media_type("audio/ogg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OggFlac)), "{:?} does not contain {}", fmt, FileFormat::OggFlac);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_ogg_opus() {
    let fmt = FileFormat::from_media_type("audio/opus");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OggOpus)), "{:?} does not contain {}", fmt, FileFormat::OggOpus);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_ogg_speex() {
    let fmt = FileFormat::from_media_type("audio/ogg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OggSpeex)), "{:?} does not contain {}", fmt, FileFormat::OggSpeex);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_ogg_vorbis() {
    let fmt = FileFormat::from_media_type("audio/ogg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OggVorbis)), "{:?} does not contain {}", fmt, FileFormat::OggVorbis);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_qualcomm_purevoice() {
    let fmt = FileFormat::from_media_type("audio/qcelp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::QualcommPurevoice)), "{:?} does not contain {}", fmt, FileFormat::QualcommPurevoice);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_quite_ok_audio() {
    let fmt = FileFormat::from_media_type("audio/x-qoa");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::QuiteOkAudio)), "{:?} does not contain {}", fmt, FileFormat::QuiteOkAudio);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_real_audio(){
    let fmt = FileFormat::from_media_type("audio/x-pn-realaudio");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Realaudio)), "{:?} does not contain {}", fmt, FileFormat::Realaudio);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_scream_tracker3_module() {
    let fmt = FileFormat::from_media_type("audio/x-s3m");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ScreamTracker3Module)), "{:?} does not contain {}", fmt, FileFormat::ScreamTracker3Module);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_sony_dsd_stream_file() {
    let fmt = FileFormat::from_media_type("audio/x-dsf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SonyDsdStreamFile)), "{:?} does not contain {}", fmt, FileFormat::SonyDsdStreamFile);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_soundfont2(){
    let fmt = FileFormat::from_media_type("audio/x-soundfont");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Soundfont2)), "{:?} does not contain {}", fmt, FileFormat::Soundfont2);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_ultimate_soundtracker_module() {
    let fmt = FileFormat::from_media_type("audio/x-mod");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UltimateSoundtrackerModule)), "{:?} does not contain {}", fmt, FileFormat::UltimateSoundtrackerModule);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_waveform_audio() {
    let fmt = FileFormat::from_media_type("audio/vnd.wave");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WaveformAudio)), "{:?} does not contain {}", fmt, FileFormat::WaveformAudio);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_wavpack(){
    let fmt = FileFormat::from_media_type("audio/wavpack");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Wavpack)), "{:?} does not contain {}", fmt, FileFormat::Wavpack);
}

#[test]
#[cfg(feature = "from-media-type")]
fn test_windows_media_audio() {
    let fmt = FileFormat::from_media_type("audio/x-ms-wma");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsMediaAudio)), "{:?} does not contain {}", fmt, FileFormat::WindowsMediaAudio);
}

