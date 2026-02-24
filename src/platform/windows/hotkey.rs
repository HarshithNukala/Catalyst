use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, hotkey::{HotKey, Modifiers, Code}};
use futures::channel::mpsc;

pub fn register_hotkey() -> mpsc::UnboundedReceiver<()> {
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::Space);
    let _ = manager.register(hotkey);
    Box::leak(Box::new(manager));

    let (tx, rx) = mpsc::unbounded::<()>();

    std::thread::spawn(move || {
        let receiver = GlobalHotKeyEvent::receiver();
        loop {
            if let Ok(_event) = receiver.recv() {
                let _ = tx.unbounded_send(());
            }
        }
    });

    rx
}