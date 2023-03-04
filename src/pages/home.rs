use leptos::*;
use crate::components::simple_counter::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {

  view! { cx,
    <div class="card my-3 mx-3 p-3">
      <h1>"Welcome to Leptos!"</h1>
    
      <SimpleCounter
        initial_value=0
        step=1
      />
    </div>
    
    <button class="btn btn-success" on:click=move |_| log!("test")>"logme"</button>
  }

}
