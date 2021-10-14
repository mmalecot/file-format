use file_format::FileFormat;

#[test]
fn test_aac() {
    let format = FileFormat::from_file("fixtures/audio/sample.aac").unwrap();
    assert_eq!(format.media_type(), "audio/aac");
    assert_eq!(format.extension(), "aac");
}

#[test]
fn test_ac3() {
    let format = FileFormat::from_file("fixtures/audio/sample.ac3").unwrap();
    assert_eq!(format.media_type(), "audio/vnd.dolby.dd-raw");
    assert_eq!(format.extension(), "ac3");
}

#[test]
fn test_aif() {
    let format = FileFormat::from_file("fixtures/audio/sample.aif").unwrap();
    assert_eq!(format.media_type(), "audio/aiff");
    assert_eq!(format.extension(), "aif");
}

#[test]
fn test_amr() {
    let format = FileFormat::from_file("fixtures/audio/sample.amr").unwrap();
    assert_eq!(format.media_type(), "audio/amr");
    assert_eq!(format.extension(), "amr");
}

#[test]
fn test_ape() {
    let format = FileFormat::from_file("fixtures/audio/sample.ape").unwrap();
    assert_eq!(format.media_type(), "audio/x-ape");
    assert_eq!(format.extension(), "ape");
}

#[test]
fn test_au() {
    let format = FileFormat::from_file("fixtures/audio/sample.au").unwrap();
    assert_eq!(format.media_type(), "audio/basic");
    assert_eq!(format.extension(), "au");
}

#[test]
fn test_f4a() {
    let format = FileFormat::from_file("fixtures/audio/sample.f4a").unwrap();
    assert_eq!(format.media_type(), "audio/mp4");
    assert_eq!(format.extension(), "f4a");
}

#[test]
fn test_f4b() {
    let format = FileFormat::from_file("fixtures/audio/sample.f4b").unwrap();
    assert_eq!(format.media_type(), "audio/mp4");
    assert_eq!(format.extension(), "f4b");
}

#[test]
fn test_flac() {
    let format = FileFormat::from_file("fixtures/audio/sample.flac").unwrap();
    assert_eq!(format.media_type(), "audio/x-flac");
    assert_eq!(format.extension(), "flac");
}

#[test]
fn test_it() {
    let format = FileFormat::from_file("fixtures/audio/sample.it").unwrap();
    assert_eq!(format.media_type(), "audio/x-it");
    assert_eq!(format.extension(), "it");
}

#[test]
fn test_m4a() {
    let format = FileFormat::from_file("fixtures/audio/sample.m4a").unwrap();
    assert_eq!(format.media_type(), "audio/x-m4a");
    assert_eq!(format.extension(), "m4a");
}

#[test]
fn test_m4b() {
    let format = FileFormat::from_file("fixtures/audio/sample.m4b").unwrap();
    assert_eq!(format.media_type(), "audio/mp4");
    assert_eq!(format.extension(), "m4b");
}

#[test]
fn test_mid() {
    let format = FileFormat::from_file("fixtures/audio/sample.mid").unwrap();
    assert_eq!(format.media_type(), "audio/midi");
    assert_eq!(format.extension(), "mid");
}

#[test]
fn test_mp3() {
    let format = FileFormat::from_file("fixtures/audio/sample.mp3").unwrap();
    assert_eq!(format.media_type(), "audio/mpeg");
    assert_eq!(format.extension(), "mp3");
}

#[test]
fn test_mpc() {
    let format = FileFormat::from_file("fixtures/audio/sample.mpc").unwrap();
    assert_eq!(format.media_type(), "audio/x-musepack");
    assert_eq!(format.extension(), "mpc");
}

#[test]
fn test_oga() {
    let format = FileFormat::from_file("fixtures/audio/sample.oga").unwrap();
    assert_eq!(format.media_type(), "audio/ogg");
    assert_eq!(format.extension(), "oga");
}

#[test]
fn test_ogg() {
    let format = FileFormat::from_file("fixtures/audio/sample.ogg").unwrap();
    assert_eq!(format.media_type(), "audio/ogg");
    assert_eq!(format.extension(), "ogg");
}

#[test]
fn test_opus() {
    let format = FileFormat::from_file("fixtures/audio/sample.opus").unwrap();
    assert_eq!(format.media_type(), "audio/opus");
    assert_eq!(format.extension(), "opus");
}

#[test]
fn test_spx() {
    let format = FileFormat::from_file("fixtures/audio/sample.spx").unwrap();
    assert_eq!(format.media_type(), "audio/ogg");
    assert_eq!(format.extension(), "spx");
}

#[test]
fn test_wav() {
    let format = FileFormat::from_file("fixtures/audio/sample.wav").unwrap();
    assert_eq!(format.media_type(), "audio/vnd.wave");
    assert_eq!(format.extension(), "wav");
}

#[test]
fn test_wv() {
    let format = FileFormat::from_file("fixtures/audio/sample.wv").unwrap();
    assert_eq!(format.media_type(), "audio/wavpack");
    assert_eq!(format.extension(), "wv");
}
