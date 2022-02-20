use os_id::{ThreadName, ThreadId};

#[test]
fn should_use_current_thread() {
    let thread1 = ThreadId::current();
    let thread2 = ThreadId::current();

    assert_eq!(thread1, thread2);
    assert_eq!(thread1.as_raw(), thread2.as_raw());

    let another_thread = std::thread::spawn(|| {
        ThreadId::current()
    }).join().unwrap();

    assert_ne!(thread1, another_thread);
    assert_ne!(thread1.as_raw(), another_thread.as_raw());
}

#[test]
fn should_get_current_thread_name() {
    std::thread::Builder::new().name("test".to_owned()).spawn(|| {
        let name = os_id::thread::get_current_thread_name();
        assert_eq!(name, "test");
    }).unwrap().join().unwrap();

    let name = os_id::thread::get_current_thread_name();
    assert_ne!(name, "test");
    assert_ne!(name.as_str(), Ok("test"));
}

#[test]
fn verify_thread_name_struct() {
    const DATA: &str = "1234567891234567";

    for idx in 0..DATA.len() {
        let mut buf = [0u8; 16];
        buf[..idx].copy_from_slice(&DATA.as_bytes()[..idx]);
        let name = ThreadName::name(buf);
        assert_eq!(name, &DATA.as_bytes()[..idx]);
        assert_eq!(name.as_str(), Ok(&DATA[..idx]));
    }
}
