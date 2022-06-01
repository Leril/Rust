use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap());

   // let (tx, rx) = mpsc::channel();
   //  let tx1 = tx.clone();
   //
   //  thread::spawn(move ||{
   //     let vals = vec![
   //         "hi".to_string(),
   //         "froms".to_string(),
   //         "first".to_string(),
   //         "thread".to_string(),
   //     ];
   //
   //      for val in vals {
   //          tx1.send(val).unwrap();
   //          thread::sleep(Duration::from_secs(1));
   //      }
   //  });
   //
   //  thread::spawn(move ||{
   //     let vals = vec![
   //         "hi".to_string(),
   //         "from".to_string(),
   //         "second".to_string(),
   //         "thread".to_string(),
   //     ];
   //
   //      for val in vals {
   //          tx.send(val).unwrap();
   //          thread::sleep(Duration::from_secs(1));
   //      }
   //  });
   //
   //  for received in rx{
   //      println!("got: {}", received);
   //  }
   //

    //
    // let (tx, rx) = mpsc::channel();
    //
    // thread::spawn(move || {
    //     let var = "hi".to_string();
    //     tx.send(var).unwrap();
    // });
    //
    // let received = rx.recv().unwrap();
    // println!("got {}", received);
    // let v = vec![1, 2, 3];
    //
    // let handle = thread::spawn(move ||{
    //     println!("Here's a vector {:?}", v);
    // });
    //
    // handle.join().unwrap();
    //
    // let handle2 = thread::spawn(||{
    //     for i in 1..10{
    //         println!("hi number {} from spawned thread", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    //
    // handle2.join().unwrap();
    //
    // for i in 1..5{
    //     println!("hi number {} from main thread", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
}
