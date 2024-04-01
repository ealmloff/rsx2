# RSX 2.0

UI is hard, we make it easier. RSX 2.0 takes your natrual language and turns it into code with an ad

```rust
use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx2! {
        "Give me buttons!!!"
    }
}
```

Now for the best part: **RSX 2.0 creates an unique UI every time you compile, ensuring your UI always stays fresh.** If you run into an get an error, just compile again it might work this time.

## Support

RSX 2.0 only supports MacOs
