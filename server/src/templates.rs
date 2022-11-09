use maud::{html, Markup, PreEscaped, DOCTYPE};

pub fn base(inner: Markup) -> Markup {
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
        link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Montserrat:wght@500;700&display=swap"

        div {
          a href="/" {
            img src="static/logo.svg" alt="Peek Logo" class="w-32 mx-auto mt-8";
          }
        }

        (inner)
      }
    }
}
