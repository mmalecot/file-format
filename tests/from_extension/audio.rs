use file_format::FileFormat;

#[test]
#[cfg(feature = "from-extension")]
fn test_adaptive_multi_rate() {
    let fmt = FileFormat::from_extension("amr");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AdaptiveMultiRate)), "{:?} does not contain {}", fmt, FileFormat::AdaptiveMultiRate);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_advanced_audio_coding() {
    let fmt = FileFormat::from_extension("aac");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AdvancedAudioCoding)), "{:?} does not contain {}", fmt, FileFormat::AdvancedAudioCoding);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_apple_itunes_audio() {
    let fmt = FileFormat::from_extension("m4a");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AppleItunesAudio)), "{:?} does not contain {}", fmt, FileFormat::AppleItunesAudio);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_apple_itunes_audiobook() {
    let fmt = FileFormat::from_extension("m4b");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AppleItunesAudiobook)), "{:?} does not contain {}", fmt, FileFormat::AppleItunesAudiobook);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_apple_itunes_protected_audio() {
    let fmt = FileFormat::from_extension("m4p");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AppleItunesProtectedAudio)), "{:?} does not contain {}", fmt, FileFormat::AppleItunesProtectedAudio);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_au(){
    let fmt = FileFormat::from_extension("au");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Au)), "{:?} does not contain {}", fmt, FileFormat::Au);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_audio_codec3() {
    let fmt = FileFormat::from_extension("ac3");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AudioCodec3)), "{:?} does not contain {}", fmt, FileFormat::AudioCodec3);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_audio_interchange_file_format() {
    let fmt = FileFormat::from_extension("aiff");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AudioInterchangeFileFormat)), "{:?} does not contain {}", fmt, FileFormat::AudioInterchangeFileFormat);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_audio_visual_research() {
    let fmt = FileFormat::from_extension("avr");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::AudioVisualResearch)), "{:?} does not contain {}", fmt, FileFormat::AudioVisualResearch);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_creative_voice() {
    let fmt = FileFormat::from_extension("voc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::CreativeVoice)), "{:?} does not contain {}", fmt, FileFormat::CreativeVoice);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_eight_bit_sampled_voice() {
    let fmt = FileFormat::from_extension("8svx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::EightBitSampledVoice)), "{:?} does not contain {}", fmt, FileFormat::EightBitSampledVoice);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_fasttracker2_extended_module() {
    let fmt = FileFormat::from_extension("xm");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Fasttracker2ExtendedModule)), "{:?} does not contain {}", fmt, FileFormat::Fasttracker2ExtendedModule);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_flash_mp4_audio() {
    let fmt = FileFormat::from_extension("f4a");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FlashMp4Audio)), "{:?} does not contain {}", fmt, FileFormat::FlashMp4Audio);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_flash_mp4_audiobook() {
    let fmt = FileFormat::from_extension("f4b");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FlashMp4Audiobook)), "{:?} does not contain {}", fmt, FileFormat::FlashMp4Audiobook);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_free_lossless_audio_codec() {
    let fmt = FileFormat::from_extension("flac");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::FreeLosslessAudioCodec)), "{:?} does not contain {}", fmt, FileFormat::FreeLosslessAudioCodec);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_impulse_tracker_module() {
    let fmt = FileFormat::from_extension("it");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ImpulseTrackerModule)), "{:?} does not contain {}", fmt, FileFormat::ImpulseTrackerModule);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_matroska_audio() {
    let fmt = FileFormat::from_extension("mka");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MatroskaAudio)), "{:?} does not contain {}", fmt, FileFormat::MatroskaAudio);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_monkeys_audio() {
    let fmt = FileFormat::from_extension("ape");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MonkeysAudio)), "{:?} does not contain {}", fmt, FileFormat::MonkeysAudio);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_mpeg12_audio_layer2() {
    let fmt = FileFormat::from_extension("mp2");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mpeg12AudioLayer2)), "{:?} does not contain {}", fmt, FileFormat::Mpeg12AudioLayer2);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_mpeg12_audio_layer3() {
    let fmt = FileFormat::from_extension("mp3");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mpeg12AudioLayer3)), "{:?} does not contain {}", fmt, FileFormat::Mpeg12AudioLayer3);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_mpeg4_part14_audio() {
    let fmt = FileFormat::from_extension("mp4");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Mpeg4Part14Audio)), "{:?} does not contain {}", fmt, FileFormat::Mpeg4Part14Audio);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_musepack(){
    let fmt = FileFormat::from_extension("mpc");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Musepack)), "{:?} does not contain {}", fmt, FileFormat::Musepack);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_musical_instrument_digital_interface() {
    let fmt = FileFormat::from_extension("mid");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::MusicalInstrumentDigitalInterface)), "{:?} does not contain {}", fmt, FileFormat::MusicalInstrumentDigitalInterface);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_ogg_flac() {
    let fmt = FileFormat::from_extension("oga");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OggFlac)), "{:?} does not contain {}", fmt, FileFormat::OggFlac);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_ogg_opus() {
    let fmt = FileFormat::from_extension("opus");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OggOpus)), "{:?} does not contain {}", fmt, FileFormat::OggOpus);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_ogg_speex() {
    let fmt = FileFormat::from_extension("spx");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OggSpeex)), "{:?} does not contain {}", fmt, FileFormat::OggSpeex);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_ogg_vorbis() {
    let fmt = FileFormat::from_extension("ogg");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::OggVorbis)), "{:?} does not contain {}", fmt, FileFormat::OggVorbis);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_qualcomm_purevoice() {
    let fmt = FileFormat::from_extension("qcp");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::QualcommPurevoice)), "{:?} does not contain {}", fmt, FileFormat::QualcommPurevoice);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_quite_ok_audio() {
    let fmt = FileFormat::from_extension("qoa");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::QuiteOkAudio)), "{:?} does not contain {}", fmt, FileFormat::QuiteOkAudio);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_real_audio(){
    let fmt = FileFormat::from_extension("ra");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Realaudio)), "{:?} does not contain {}", fmt, FileFormat::Realaudio);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_scream_tracker3_module() {
    let fmt = FileFormat::from_extension("s3m");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::ScreamTracker3Module)), "{:?} does not contain {}", fmt, FileFormat::ScreamTracker3Module);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_sony_dsd_stream_file() {
    let fmt = FileFormat::from_extension("dsf");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::SonyDsdStreamFile)), "{:?} does not contain {}", fmt, FileFormat::SonyDsdStreamFile);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_soundfont2(){
    let fmt = FileFormat::from_extension("sf2");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Soundfont2)), "{:?} does not contain {}", fmt, FileFormat::Soundfont2);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_ultimate_soundtracker_module() {
    let fmt = FileFormat::from_extension("mod");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::UltimateSoundtrackerModule)), "{:?} does not contain {}", fmt, FileFormat::UltimateSoundtrackerModule);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_waveform_audio() {
    let fmt = FileFormat::from_extension("wav");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WaveformAudio)), "{:?} does not contain {}", fmt, FileFormat::WaveformAudio);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_wavpack(){
    let fmt = FileFormat::from_extension("wv");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::Wavpack)), "{:?} does not contain {}", fmt, FileFormat::Wavpack);
}

#[test]
#[cfg(feature = "from-extension")]
fn test_windows_media_audio() {
    let fmt = FileFormat::from_extension("wma");
    assert!(fmt.is_some_and(|types| types.contains(&FileFormat::WindowsMediaAudio)), "{:?} does not contain {}", fmt, FileFormat::WindowsMediaAudio);
}

