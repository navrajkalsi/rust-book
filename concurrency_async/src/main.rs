use std::time::Duration;

fn main() {
    // block_on start the async runtime(tokio in case of trpl)
    // and runs the provided async block
    trpl::block_on(async {
        // spawn_task is used to create a new async task
        // this task runs CONCURRENTLY, not parallely, as the runtime will switch the control of
        // the program from one part to another, but cannot run both loop at once
        // PARALLELLSIM is achieved via threads
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                // this sleep await lets the control move to other part,
                // like the next loop
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            // when this loop awaits, the control can move to any loop that is ready
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // as soon as the second loop exhausts the spawned task will also be terminated as we are
        // now leaving this scope
        // so we need to await the handle of the spawned task for it to finish.
        // NOTE this will NOT block like threads, as we are calling await
        handle.await.unwrap();
    });

    println!("\nnow without spawning a new task.\n");

    trpl::block_on(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        // we just await for both the futures to join with trpl::join
        // instead calling await on both of them
        trpl::join(fut1, fut2).await;
        // trpl::join function is fair, meaning it checks each future equally often,
        // alternating between them, and never lets one race ahead if the other is ready.
        // although this is NOT A GUARANTEE
        // With threads, the operating system decides which thread to check and how long to let it run.
    });

    println!("\ntrpl channel demo, not utilizing full async\n");

    trpl::block_on(async {
        // The synchronous Receiver::recv method in std::mpsc::channel blocks until it receives a message.
        // The trpl::Receiver::recv method does not, because it is async. Instead of blocking,
        // it hands control back to the runtime until either a message is received or the send side of the channel closes.
        // By contrast, we don’t await the send call, because it doesn’t block.
        // It doesn’t need to, because the channel we’re sending it into is unbounded.
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // a temp fix, see the next implementation on why this is necessary and what is the best
        // way to do this
        drop(tx);

        // while let lets us await till one of the recv calls result in None
        // the behaviour is just like 'if let'
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }

        // NOTE: this block is just one async block,
        // that is, transmitting and receiving is being done in one async block
        // the code runs linearly and because of the await when send() is called
        // the runtime is given back the control so that it can perform other tasks that
        // are ready INSIDE OTHER ASYNC BLOCKS, not receiving as the runtime cannot reach it yet
        // because it is unable to proceed ahead of the send call
        //
        // the recv calls in the while loop will be fulfilled instantly when the control gets
        // there, after transmitting all the data with send
    });

    println!("\ntrpl channel demo, done right in full async\n");

    // doing async transmit and receive the right way
    // by placing the transmittion and reception of the data in two separate blocks,
    // so that when we call await in one block,
    // the control can go the other block to recv() or send()
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        // NOTE here we use async move
        // this moves the ownership of tx, which is then dropped when this async block ends
        // this is necessary as this then lets us receive None in the 'while let' loop during
        // recv(), which lets us exit the loop
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });

    println!("\ntrpl channel demo, with multiple transmitters/producers\n");

    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        // join! is a macro that joins an arbitrary number of futures
        // but the number of futures has to be known at compile time for join! to work
        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });

    // async code runs in an async block in a trpl::block_on call, everything within it can avoid blocking.
    // However, the code outside it will block on the block_on function returning.
    // That’s the whole point of the trpl::block_on function:
    // it lets you choose where to block on some set of async code, and thus where to transition between sync and async code.
}
