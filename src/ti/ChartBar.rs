#[cfg(feature = "TiChartBar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiChartBar")]
/// *This icon requires the feature* `TiChartBar` *to be enabled*.
#[component]
pub fn ChartBar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M14 4c0-1.105-.896-2-2-2s-2 .895-2 2v12h4v-12zM19 8c0-1.105-.896-2-2-2s-2 .895-2 2v8h4v-8zM9 11c0-1.105-.896-2-2-2s-2 .895-2 2v5h4v-5zM19 19h-14c-.553 0-1 .447-1 1s.447 1 1 1h14c.553 0 1-.447 1-1s-.447-1-1-1z" /></svg>
   }
}