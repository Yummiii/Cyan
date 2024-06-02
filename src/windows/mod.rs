use std::{process, sync::mpsc};
use tray_item::{IconSource, TrayItem};

pub mod screenshots;

enum Message {
    Quit,
}

pub fn to_tray() {
    let mut tray = TrayItem::new("Cyan", IconSource::Resource("aa-exe-icon")).unwrap();

    let (tx, rx) = mpsc::sync_channel(1);
    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    loop {
        if let Ok(Message::Quit) = rx.recv() {
            println!("Quit");
            process::exit(0);
        }
        // match rx.recv() {
        //     Ok(Message::Quit) => {
        //         println!("Quit");
        //         process::exit(0);
        //     }
        //     _ => {}
        // }
    }
}
