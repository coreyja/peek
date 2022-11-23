use maud::{html, Markup, Render};

fn shared_button_classes() -> &'static str {
    "text-xl block my-2 py-2 font-bold text-center font-sans rounded-lg w-full"
}

pub(crate) fn primary_button_classes() -> String {
    let base = shared_button_classes();
    format!("{base} bg-[#CADFFF] text-[#001571]")
}

pub(crate) fn secondary_button_classes() -> String {
    let base = shared_button_classes();
    format!("{base} bg-[#001571] text-[#CADFFF]")
}

pub(crate) fn primary_link_button<Content: Render>(contents: Content, href: &str) -> Markup {
    html! {
        a
            href=(href)
            class=(primary_button_classes())
            { (contents) }
    }
}

pub(crate) fn submit_button(contents: &str) -> Markup {
    html! {
        input
            type="submit"
            class=(primary_button_classes())
            value=(contents);
    }
}

pub(crate) fn secondary_link_button<Content: Render>(contents: Content, href: &str) -> Markup {
    html! {
        a
            href=(href)
            class=(secondary_button_classes())
            { (contents) }
    }
}

pub(crate) fn signout_button() -> Markup {
    html! {
        form method="post" action="/sign-out" {
            (submit_button("Sign Out"))
        }
    }
}
