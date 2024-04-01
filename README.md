# RSX 2.0

UI is hard, we make it easier. RSX 2.0 takes your natrual language and turns it into code with an advanced AI we call MT (monkey with a typewriter)

```rust
use dioxus::prelude::*;
use rsx2::rsx2;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx2! {
        "Give me buttons!!!"
    }
}
```

Now for the best part: **RSX 2.0 creates a unique UI every time you compile, ensuring your UI always stays fresh.** If you run into an get an error, just compile again (it might work this time)

## Support

RSX 2.0 supports the minimal subset of platforms and HTML: MacOS and buttons

No further elements will be supported, buttons are sufficient for all use cases. If you develop on MacOS, but would like to support users on other platforms, we recommend you ship your computer to the client.
