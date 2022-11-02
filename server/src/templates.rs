use maud::{html, Markup, PreEscaped, DOCTYPE};

pub fn base(inner: Markup) -> Markup {
    html! {
      (DOCTYPE)
      html lang="en" class="bg-blue-500" {
        head {
          meta name="viewport" content="width=device-width, initial-scale=1.0";
          title { "Peek" }
        }
        script type="module" {
          (PreEscaped(r#"
            // Use ES module import syntax to import functionality from the module
            // that we have compiled.
            //
            // Note that the `default` import is an initialization function which
            // will "boot" the module and make it ready to use. Currently browsers
            // don't support natively imported WebAssembly as an ES module, but
            // eventually the manual initialization won't be required!
            import init, { add } from './static/frontend.js';

            async function run() {
              // First up we need to actually load the wasm file, so we use the
              // default export to inform it where the wasm file is located on the
              // server, and then we wait on the returned promise to wait for the
              // wasm to be loaded.
              //
              // It may look like this: `await init('./pkg/without_a_bundler_bg.wasm');`,
              // but there is also a handy default inside `init` function, which uses
              // `import.meta` to locate the wasm file relatively to js file.
              //
              // Note that instead of a string you can also pass in any of the
              // following things:
              //
              // * `WebAssembly.Module`
              //
              // * `ArrayBuffer`
              //
              // * `Response`
              //
              // * `Promise` which returns any of the above, e.g. `fetch("./path/to/wasm")`
              //
              // This gives you complete control over how the module is loaded
              // and compiled.
              //
              // Also note that the promise, when resolved, yields the wasm module's
              // exports which is the same as importing the `*_bg` module in other
              // modes
              await init("static/frontend_bg.wasm");

              // And afterwards we can use all the functionality defined in wasm.
              const result = add(1, 2);
              console.log(`1 + 2 = ${result}`);
              if (result !== 3)
                throw new Error("wasm addition doesn't work!");
            }

            run();
            "#))
        }
        link rel="stylesheet" href="static/tailwind.css";
        (inner)
      }
    }
}
