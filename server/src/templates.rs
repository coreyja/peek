use maud::{html, Markup, DOCTYPE};

pub fn base(inner: Markup) -> Markup {
    html! {
      (DOCTYPE)
      html lang="en" {
        head {
          meta name="viewport" content="width=device-width, initial-scale=1.0";
          title { "YANWA" }
        }
        (inner)
      }
    }
}
