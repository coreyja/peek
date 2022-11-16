use maud::{html, Markup};

pub(crate) fn form_input(name: &str, label: &str, input_type: &str) -> Markup {
    html! {
      label class="block mb-8" {
        div class="pb-2" { (label) }
        input
          type=(input_type)
          name=(name)
          required="required"
          placeholder=(label)
          class="block w-full p-2 border border-[#CADFFF] rounded"
          ;
      }
    }
}
