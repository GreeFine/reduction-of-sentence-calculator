use web_sys::Event;
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct InputComponentProps {
    #[prop_or_default]
    pub onchange: Callback<Event>,
    pub itype: &'static str,
    pub name: Option<&'static str>,
    pub value: Option<String>,
    pub checked: Option<bool>,
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
              if let Some(name) = ctx.props().name {
                <span>{name}</span>
              }
              if let Some(value) = ctx.props().value.as_ref() {
                <input value={value.clone()} onchange={&ctx.props().onchange} type={ctx.props().itype} />
              } else {
                <input checked={ctx.props().checked.unwrap_or(false)} onchange={&ctx.props().onchange} type={ctx.props().itype} />
              }
              <br />
            </div>
        }
    }
}
