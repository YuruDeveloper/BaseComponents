use yew::prelude::*;

use crate::Components::Button::Button;


#[function_component]
pub fn Header() -> Html {
    html!{
        <header class="bg-white shadow-sm">
            <div>
                <div class="flex flex-col items-start">
                    <h1 class="text-3xl font-bold text-gray-900 mb-4 hover:text-gray-800 transition-colors">
                        <a href="/"></a>
                    </h1>
                    <nav class="flex space-x-6 ">
                        <Button Text="AboutMe" Link="/aboutme" Icon="fas fa-mug-hot"/>
                        <Button Text="Posts" Link="/posts" Icon="fas fa-newspaper mr-1"/>
                        <Button Text="Products" Link="/products" Icon="fas fa-truck mr-1"/>
                        <Button Text="LogIn" Link="/login" Icon="fas fa-tree"/>
                        <Button Text="Write" Link="/write" Icon="fas fa-pen-to-quare"/>
                        <Button Text="Upload" Link="/upload" Icon="fas fa-arrow-up-from-bracket"/>
                        <Button Text="Logs" Link="/logs" Icon="fas fa-database"/>
                    </nav> 
                </div>
            </div>
        </header>
    }
}