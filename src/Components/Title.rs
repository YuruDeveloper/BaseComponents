use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct Props{
    Title : String
}

#[function_component]
pub fn Title(Props : &Props) -> Html {
    html!{
        <div class="relative mb-12">
            <h2 class="text-3xl font-bold text-center relative z-10">
                <span class="absolute inset-x-0 bottom-0 h-1 bg-gradient-to-r from-gray-900 to-gray-100">{&Props.Title}</span>
            </h2>
            <div class="absolute inset-x-0 bottom-0 h-1 bg-gradient-to-r from-gray-900 to-gray-100 transform translate-y-2"/>
        </div>
    }
}