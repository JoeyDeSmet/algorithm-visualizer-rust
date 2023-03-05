use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::home::*;
use crate::pages::counter_page::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  provide_meta_context(cx);

  view! {
    cx,

    <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
    <Title text="Algorithm visualizer"/>

    <Router>
      <main>
        <Routes>
        
          <Route path="" view=|cx| view! { cx, 
            <HomePage/> 
          }/>
        
          <Route path="/counters" view=|cx| view! { cx,
            <CounterPage/>
          }/>

        </Routes>
      </main>
    </Router>
  }
}
