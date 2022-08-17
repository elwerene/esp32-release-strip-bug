fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut buf = [0u8; 254];
    let len = cobs::try_encode(&[0, 0, 71], &mut buf).unwrap();
    if buf[..len] == [1, 1, 2, 71] {
        println!("debug build :)");
    } else {
        println!("release build :/");
    }
}
