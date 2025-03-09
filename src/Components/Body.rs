use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct Props{
    pub Children : Html
}

#[function_component]
pub fn Body(Props : &Props) -> Html{
    html!{
        <main class="container mx-auto max-w-6xl px-4 py-8">
            { Props.Children.clone()}
        </main>
    }
}