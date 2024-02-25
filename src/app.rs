use leptos::*;

#[component]
pub fn App() -> impl IntoView{
    view!{
        <div class="container">
            <input type="text" placeholder="Todo"/>
            <button class="add">Add</button>
        </div>
    }
}
