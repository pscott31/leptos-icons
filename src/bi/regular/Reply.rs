#[cfg(feature = "BiRegularReply")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularReply")]
/// *This icon requires the feature* `BiRegularReply` *to be enabled*.
#[component]
pub fn Reply(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10 11h6v7h2v-8a1 1 0 0 0-1-1h-7V6l-5 4 5 4v-3z" /></svg>
   }
}