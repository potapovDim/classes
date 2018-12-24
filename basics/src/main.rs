#[derive(Debug)]
enum WebEvent {
  PageLoad,
  Pageunload,
  KeyPress(char),
  Paste(String),
  Click { x: i64, y: i64 },
}

fn inspect_web_element(event: WebEvent) {
  match event {
    WebEvent::KeyPress(c) => println!("Key {} was pressed ", c),
    WebEvent::Paste(s) => println!("String was pasted {}", s),
    WebEvent::PageLoad => println!("Page loading "),
    WebEvent::Pageunload => println!("Page loaded"),
    WebEvent::Click { x, y } => println!("Click was executed on x: {}, y: {}", x, y),
  }
}

fn main() {
  let click = WebEvent::Click{x:10, y: 12};
  inspect_web_element(click);
}
