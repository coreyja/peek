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
        link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Montserrat";

        div {
          img src="static/logo.svg" alt="Peek Logo" class="w-32 mx-auto mt-8";
          h1 class="text-center my-8 font-serif text-2xl text-[#001571] font-bold" { "Weather & News Updates" }
          img src="static/hero.png" alt="Peek Hero" class="my-8";

          p class="font-sans my-8 px-8 text-center leading-relaxed text-[#000620] text-2xl" {
            "Taking a peek at local weather and news, keeps you connected with your long distance coworkers."
          }
        }
        (inner)
      }
    }
}
