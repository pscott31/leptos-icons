#[cfg(feature = "TbRating21Plus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbRating21Plus")]
/// *This icon requires the feature* `TbRating21Plus` *to be enabled*.
#[component]
pub fn Rating21Plus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-rating-21-plus" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /><path d="M13 15v-6" /><path d="M15.5 12h3" /><path d="M17 10.5v3" /><path d="M7 10.5a1.5 1.5 0 0 1 3 0c0 .443 -.313 .989 -.612 1.393l-2.388 3.107h3" /></svg>
   }
}