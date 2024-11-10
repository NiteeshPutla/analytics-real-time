mod avl_tree;

use avl_tree::AVLNode;
use std::sync::{Arc, Mutex};
use crossbeam::channel;

fn main() {
    let data_store = Arc::new(Mutex::new(Vec::<i32>::new()));
    let (sender, receiver) = channel::unbounded();

    let data_store_clone = Arc::clone(&data_store);
    std::thread::spawn(move || {
        while let Ok(data) = receiver.recv() {
            let mut store = data_store_clone.lock().unwrap();
            store.push(data);
            println!("Received and processed data: {}", data);
        }
    });

    for i in 0..100 {
        sender.send(i).unwrap();
    }
}