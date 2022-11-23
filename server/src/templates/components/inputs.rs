use maud::{html, Markup, Render};

pub(crate) struct FormInputOptions {
    pub(crate) input_type: &'static str,
    pub(crate) required: bool,
}

impl Default for FormInputOptions {
    fn default() -> Self {
        Self {
            input_type: "text",
            required: false,
        }
    }
}
pub(crate) struct FormInput {
    pub(crate) label: &'static str,
    pub(crate) name: &'static str,
    pub(crate) options: FormInputOptions,
}

impl FormInput {
    pub(crate) fn simple(label: &'static str, name: &'static str) -> Self {
        Self {
            label,
            name,
            options: Default::default(),
        }
    }

    pub(crate) fn required(label: &'static str, name: &'static str) -> Self {
        Self {
            label,
            name,
            options: FormInputOptions {
                required: true,
                ..Default::default()
            },
        }
    }
}

impl Render for FormInput {
    fn render(&self) -> Markup {
        let required_value = if self.options.required {
            Some("required")
        } else {
            None
        };

        html! {
          label class="block mb-8" {
            div class="pb-2" { (self.label) }
            input
              type=(self.options.input_type)
              name=(self.name)
              required=[required_value]
              placeholder=(self.label)
              class="block w-full p-2 border border-[#CADFFF] rounded"
              ;
          }
        }
    }
}
