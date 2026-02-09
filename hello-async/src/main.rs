use trpl::{Either, Html};

fn main() {
    // main can’t be marked async is that async code needs a runtime:
    // a Rust crate that manages the details of executing asynchronous code.
    // A program’s main function can initialize a runtime, but it’s not a runtime itself.
    // Every Rust program that executes async code has at least one place where it sets up a runtime that executes the futures.
    let args: Vec<String> = std::env::args().collect();

    // here the runtime(tokio) decides where to transfer the control to
    // like in epoll with c, this was done manually with state management
    // when the runtime detects that a function is ready for a particular state
    // the control is transferred back to the function by the runtime

    // the rust compiler manages the state machine aspect automatically

    // calling block_on sets up a runtime using the tokio crate that’s used to run the future passed in
    // Once the future completes, block_on returns whatever value the future produced.
    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    });

    // fetching two pages at once
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        // futures are lazy, these calls to v2 function do not anything and will not be in the
        // interest list of the tokio runtime at this time
        let title_fut_1 = page_title_v2(&args[1]);
        let title_fut_2 = page_title_v2(&args[2]);

        let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}

async fn page_title(url: &str) -> Option<String> {
    // this fetches the data in async fashion
    let response = trpl::get(url).await;
    // this converts the data to text in asyn fashion
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

async fn page_title_v2(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
