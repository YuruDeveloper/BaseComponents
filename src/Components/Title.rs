use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct Props{
    pub Title : String
}

#[function_component]
pub fn Title(Props : &Props) -> Html {
    html!{
        <div class="relative mb-12">
            <h2 class="text-3xl font-bold text-center relative z-10">
                <span class="bg-slate-700 bg-clip-text text-transparent">{&Props.Title}</span>
            </h2>
            <div class="absolute inset-x-0 bottom-0 h-1 bg-slate-700 transform translate-y-2"/>
        </div>
    }
}