use os_id::ThreadId;

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

#[cfg(feature = "thread-name")]
#[test]
fn should_get_current_thread_name() {
    std::thread::Builder::new().name("test".to_owned()).spawn(|| {
        let name = os_id::thread::get_current_thread_name();
        assert_eq!(name, "test");
    }).unwrap().join().unwrap();

    let name = os_id::thread::get_current_thread_name();
    assert_ne!(name, "test");
}
