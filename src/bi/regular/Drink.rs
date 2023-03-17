#[cfg(feature = "BiRegularDrink")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDrink")]
/// *This icon requires the feature* `BiRegularDrink` *to be enabled*.
#[component]
pub fn Drink(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20.832 4.555A1 1 0 0 0 20 3H4a1 1 0 0 0-.832 1.554L11 16.303V20H8v2h8v-2h-3v-3.697l7.832-11.748zM12 14.197 8.535 9h6.93L12 14.197zM18.132 5l-1.333 2H7.201L5.868 5h12.264z" /></svg>
   }
}