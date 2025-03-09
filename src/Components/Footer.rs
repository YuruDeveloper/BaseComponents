use yew::prelude::*;

#[function_component]
pub fn Footer()-> Html{
    html!{
        <footer class="bg-white shadow-inner mt-12">
            <div class="container mx-auto max-w-6xl px-4 py-6 text-sm text-gray-500">
                <p>{"© 2025 Yuru Developer"}</p>
            </div>
        </footer>
    }
}