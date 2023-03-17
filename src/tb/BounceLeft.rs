#[cfg(feature = "TbBounceLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBounceLeft")]
/// *This icon requires the feature* `TbBounceLeft` *to be enabled*.
#[component]
pub fn BounceLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-bounce-left" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M20 15.5c-3 -1 -5.5 -.5 -8 4.5c-.5 -3 -1.5 -5.5 -3 -8" /><path d="M6 9a2 2 0 1 1 0 -4a2 2 0 0 1 0 4z" /></svg>
   }
}