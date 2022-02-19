use web_sys::Event;
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct InputComponentProps {
    #[prop_or_default]
    pub onchange: Callback<Event>,
    pub itype: &'static str,
    pub name: &'static str,
}

pub struct InputComponent;

impl Component for InputComponent {
    type Message = ();
    type Properties = InputComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
              <span>{ctx.props().name}</span>
              <input onchange={&ctx.props().onchange} type={ctx.props().itype} />
              <br />
            </div>
        }
    }
}
