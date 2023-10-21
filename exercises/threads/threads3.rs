// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}
// 修改send_tx函数，接受Arc<Queue>和mpsc::Sender<u32>
fn send_tx(q: Arc<Queue>, tx: mpsc::Sender<u32>) {
    let qc1 = Arc::clone(&q);
    let qc2 = Arc::clone(&q);

    // 为每个线程创建新的mpsc::Sender<u32>副本
    let tx1 = tx.clone();
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // 为每个线程创建新的mpsc::Sender<u32>副本
    let tx2 = tx.clone();
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Queue::new());
    let queue_length = queue.length;

    // 将Arc<Queue>传递给send_tx，并传递给send_tx的每个线程
    send_tx(Arc::clone(&queue), tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue_length {
            break; // 接收所有元素后退出循环
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}



