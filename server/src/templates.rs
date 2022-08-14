use maud::{html, Markup};

pub fn base(inner: Markup) -> Markup {
    html! {
      html {
        head {
          meta name="viewport" content="width=device-width, initial-scale=1.0";
          title { "YANWA" }
        }
        (inner)
      }
    }
}
