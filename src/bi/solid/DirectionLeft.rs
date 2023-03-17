#[cfg(feature = "BiSolidDirectionLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDirectionLeft")]
/// *This icon requires the feature* `BiSolidDirectionLeft` *to be enabled*.
#[component]
pub fn DirectionLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m21.707 11.294-8.978-9a1.001 1.001 0 0 0-1.415-.002l-9.021 9a1 1 0 0 0 0 1.416l9.021 9c.39.389 1.026.388 1.415-.002l8.978-9a.998.998 0 0 0 0-1.412zM15 16h-2v-4h-3v2l-3-3 3-3v2h5v6z" /></svg>
   }
}