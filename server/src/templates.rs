use maud::{html, Markup, PreEscaped, DOCTYPE};

pub fn base(inner: Markup, with_footer: bool) -> Markup {
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
            div class="flex-grow overflow-y-scroll" {
              div {
                a href="/" {
                  img src="static/logo.svg" alt="Peek Logo" class="w-32 mx-auto mt-8";
                }
              }
              (inner)
            }

            @if with_footer { (footer::footer()) }
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
            Icon::Home => "fa-solid fa-house-chimney fa-lg",
            Icon::AddCircle => "fa-regular fa-circle-plus fa-lg",
            Icon::Profile => "fa-regular fa-user fa-lg",
        }
    }
}

mod footer {
    use maud::{html, Markup, Render};

    use super::Icon;

    pub(crate) fn footer() -> Markup {
        html! {
          div class="bg-[#CADFFF] h-16 rounded-lg flex flex-row" data-testid="footer" {
            (FooterItem::new("Home", Icon::Home, "/home"))
            (FooterItem::new("Add", Icon::AddCircle, "/team_members"))
            (FooterItem::new("Profile", Icon::Profile, "/profile"))
          }
        }
    }

    struct FooterItem<'a, 'b> {
        label: &'a str,
        icon: Icon,
        href: &'b str,
    }

    impl<'a, 'b> FooterItem<'a, 'b> {
        fn new(label: &'a str, icon: Icon, href: &'b str) -> Self {
            Self { label, icon, href }
        }
    }

    impl<'a, 'b> Render for FooterItem<'a, 'b> {
        fn render(&self) -> Markup {
            html! {
              a href=(self.href) class="flex-1 flex flex-col items-center justify-center" {
                i class=(self.icon.to_font_awesome_class()) {}
                p { (self.label) }
              }
            }
        }
    }
}
