use yew::prelude::*;

use crate::Components::Button::Button;


#[function_component]
pub fn Header() -> Html {
    html!{
        <header class="bg-white shadow-sm">
            <div class="container mx-auto max-w-6xl px-4 py-4">
                <div class="flex flex-col items-start">
                    <h1 class="text-3xl font-bold text-slate-700 mb-4 hover:text-slate-500 transition-colors">
                        <a href="/">{"Yuru Developer"}</a>
                    </h1>
                    <nav class="flex space-x-6 ">
                        <Button Text="AboutMe" Link="/aboutme" Icon="fa-solid fa-mug-hot"/>
                        <Button Text="Posts" Link="/posts" Icon="fa-solid fa-newspaper mr-1"/>
                        <Button Text="Products" Link="/products" Icon="fa-solid fa-truck mr-1"/>
                        <Button Text="LogIn" Link="/login" Icon="fa-solid fa-tree"/>
                        <Button Text="Write" Link="/write" Icon="fa-solid fa-pen-to-squre"/>
                        <Button Text="Upload" Link="/upload" Icon="fa-solid fa-arrow-up-from-bracket"/>
                        <Button Text="Logs" Link="/logs" Icon="fa-solid fa-database"/>
                    </nav> 
                </div>
            </div>
        </header>
    }
}