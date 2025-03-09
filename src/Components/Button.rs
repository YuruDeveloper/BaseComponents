use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct Props{
    pub Link : String,
    pub Text : String,
    pub Icon : String,
}

#[function_component]
pub fn Button(props : &Props)->Html{
    html!{
        <a href={props.Link.clone()} class="text-slate-700 hover:text-slate-500 transition-colors flex items-center">
            <i class={&props.Icon}></i> {&props.Text}
        </a>
    }
}