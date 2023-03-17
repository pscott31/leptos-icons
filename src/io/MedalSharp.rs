#[cfg(feature = "IoMedalSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoMedalSharp")]
/// *This icon requires the feature* `IoMedalSharp` *to be enabled*.
#[component]
pub fn MedalSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="80 32 16 160 305.11 160 385.33 32 80 32" /><path d="M496,144,424,32,298,231.08a128,128,0,0,0-84,0L189.53,192H43.82l86.66,134.89a128,128,0,1,0,251,0ZM256,422a70,70,0,1,1,70-70A70.08,70.08,0,0,1,256,422Z" /><circle cx="256" cy="352" r="32" /></svg>
   }
}