use os_id::ProcessId;

#[test]
fn should_use_current_process() {
    let pid1 = ProcessId::current();
    let pid2 = ProcessId::current();

    assert_eq!(pid1, pid2);
    assert_eq!(pid1.as_raw(), pid2.as_raw());

    let another_thread = std::thread::spawn(|| {
        ProcessId::current()
    }).join().unwrap();

    assert_eq!(pid1, another_thread);
    assert_eq!(pid1.as_raw(), another_thread.as_raw());
    assert_eq!(pid1.to_string(), another_thread.to_string());
}
