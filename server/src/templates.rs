use maud::{html, Markup, PreEscaped, DOCTYPE};

use self::footer::Footer;

pub(crate) mod footer;
pub(crate) mod components {
    pub(crate) mod buttons;
    pub(crate) mod inputs;
}

pub(crate) fn base(inner: Markup, footer: Option<Footer>) -> Markup {
    html! {
      (DOCTYPE)
      // TODO: Move this to a Tailwind color
      html lang="en" class="bg-[#FAFCFF]" {
        head {
          meta name="viewport" content="width=device-width, initial-scale=1.0";
          title { "Peek" }
        }
        script type="module" {
          (PreEscaped(r#"
            import init, { add } from './pkg/frontend.js';

            async function run() {
              await init("pkg/frontend_bg.wasm");

              const result = add(1, 2);
              console.log(`1 + 2 = ${result}`);
              if (result !== 3)
                throw new Error("wasm addition doesn't work!");
            }

            run();
            "#))
        }
        link rel="stylesheet" href="pkg/tailwind.css";
        link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Merriweather";
        link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Montserrat:wght@500;700&display=swap";

        script src="https://kit.fontawesome.com/aeb22c2a3e.js" crossorigin="anonymous" {}

        body {
          div class="h-screen flex flex-col" {
            div class="flex-grow overflow-y-scroll px-8" {
              div {
                a href="/" {
                  img src="static/logo.svg" alt="Peek Logo" class="w-32 mx-auto mt-8";
                }
              }
              (inner)
            }

            @if let Some(footer) = footer { (footer) }
          }
        }
      }
    }
}

enum Icon {
    Home,
    AddCircle,
    Profile,
}

impl Icon {
    const fn to_font_awesome_class(&self) -> &'static str {
        match self {
            Icon::Home => "fa-regular fa-house-chimney fa-lg",
            Icon::AddCircle => "fa-regular fa-circle-plus fa-lg",
            Icon::Profile => "fa-regular fa-user fa-lg",
        }
    }

    const fn to_active_font_awesome_class(&self) -> &'static str {
        match self {
            Icon::Home => "fa-solid fa-house-chimney fa-lg",
            Icon::AddCircle => "fa-solid fa-circle-plus fa-lg",
            Icon::Profile => "fa-solid fa-user fa-lg",
        }
    }
}
