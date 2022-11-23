use enum_iterator::{all, Sequence};
use maud::{html, Markup, Render};

use super::Icon;

pub(crate) struct Footer {
    active_item: FooterItem,
}

impl Footer {
    pub fn new(active_item: FooterItem) -> Self {
        Self { active_item }
    }
}

impl Default for Footer {
    fn default() -> Self {
        Self {
            active_item: FooterItem::Home,
        }
    }
}

impl Render for Footer {
    fn render(&self) -> Markup {
        html! {
          div class="bg-[#CADFFF] h-16 rounded-lg flex flex-row" data-testid="footer" {
            @for item in all::<FooterItem>() {
              (FooterItemWithActive { active: item == self.active_item, item })
            }
          }
        }
    }
}

#[derive(Sequence, PartialEq)]
pub(crate) enum FooterItem {
    Home,
    Add,
    Profile,
}

struct FooterItemWithActive {
    item: FooterItem,
    active: bool,
}

impl Render for FooterItemWithActive {
    fn render(&self) -> Markup {
        html! {
          a href=(self.item.href()) class="flex-1 flex flex-col items-center justify-center" {
            @if self.active {
              i class=(self.item.icon().to_active_font_awesome_class()) {}
            } @else {
              i class=(self.item.icon().to_font_awesome_class()) {}
            }
            p { (self.item.label()) }
          }
        }
    }
}

impl FooterItem {
    const fn icon(&self) -> Icon {
        match self {
            FooterItem::Home => Icon::Home,
            FooterItem::Add => Icon::AddCircle,
            FooterItem::Profile => Icon::Profile,
        }
    }

    const fn label(&self) -> &'static str {
        match self {
            FooterItem::Home => "Home",
            FooterItem::Add => "Add",
            FooterItem::Profile => "Profile",
        }
    }

    const fn href(&self) -> &'static str {
        match self {
            FooterItem::Home => "/home",
            FooterItem::Add => "/team_members",
            FooterItem::Profile => "/profile",
        }
    }
}
