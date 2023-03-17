#[cfg(feature = "BiRegularMessageSquareCheck")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMessageSquareCheck")]
/// *This icon requires the feature* `BiRegularMessageSquareCheck` *to be enabled*.
#[component]
pub fn MessageSquareCheck(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16 2H8C4.691 2 2 4.691 2 8v13a1 1 0 0 0 1 1h13c3.309 0 6-2.691 6-6V8c0-3.309-2.691-6-6-6zm4 14c0 2.206-1.794 4-4 4H4V8c0-2.206 1.794-4 4-4h8c2.206 0 4 1.794 4 4v8z" /><path d="m11 13.586-2.293-2.293-1.414 1.414L11 16.414l6.207-6.207-1.414-1.414z" /></svg>
   }
}