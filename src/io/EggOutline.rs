#[cfg(feature = "IoEggOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoEggOutline")]
/// *This icon requires the feature* `IoEggOutline` *to be enabled*.
#[component]
pub fn EggOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C192,48,96,171.69,96,286.55S160,464,256,464s160-62.59,160-177.45S320,48,256,48Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}