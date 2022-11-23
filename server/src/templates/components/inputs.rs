use maud::{html, Markup};

pub(crate) fn form_input(name: &str, label: &str, input_type: &str, required: bool) -> Markup {
    let required_value = if required { Some("required") } else { None };
    html! {
      label class="block mb-8" {
        div class="pb-2" { (label) }
        input
          type=(input_type)
          name=(name)
          required=[required_value]
          placeholder=(label)
          class="block w-full p-2 border border-[#CADFFF] rounded"
          ;
      }
    }
}
