use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        // streams are same as iterators, but asynchronous
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        // to give the control back to rust explicitly, we can call yield_now()
        // and then the runtime can look for other things to execute
        // used to voluntarily suspend the current task
        // this DOES NOTHING HERE as we are only working with one block
        trpl::yield_now().await;

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
