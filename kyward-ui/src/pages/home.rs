use yew::prelude::*;
use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize::Four;

pub enum Msg {
    AddOne,
}

pub struct Home {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        // https://bulma.io/documentation/overview/start/
        html! {
            <ybc::Container fluid=true>
              <ybc::Tile ctx=Ancestor>
                <ybc::Tile ctx=Parent vertical=true size=Four>
                  <ybc::Tile ctx=Child classes=classes!("box")>
                    <p>{"Lorem ipsum dolor sit amet ..."}</p>
                    <ybc::Button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</ybc::Button>
                    <p>{ self.value }</p>
                    <a href="/two">{"two"}</a>
                  </ybc::Tile>
                  /* .. snip .. more tiles here .. */
                </ybc::Tile>
              </ybc::Tile>
            </ybc::Container>
        }
    }
}