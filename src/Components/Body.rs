use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct Props{
    pub children : Html
}

#[function_component]
pub fn Body(Props : &Props) -> Html{
    html!{
        <main class="container mx-auto max-w-6xl px-4 py-8">
            { Props.children.clone()}
        </main>
    }
}