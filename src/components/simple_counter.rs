use leptos::*;

#[component]
pub fn SimpleCounter(
  cx: Scope,
  initial_value: i32,
  step: i32
) -> impl IntoView {

  let (value, set_value) = create_signal(cx, initial_value);

  view! { cx,
    <div>
      <button 
        class="btn btn-success"
        on:click=move |_| set_value.update(|value| *value -= step)
      >
        "-1"
      </button>
      
      <span class="mx-2">"Value: " {value} "!"</span>
      
      <button 
        class="btn btn-success"
        on:click=move |_| set_value.update(|value| *value += step)
      >
        "+1"
      </button>      
    </div>
  }

}
