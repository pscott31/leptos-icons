#[cfg(feature = "BiSolidUserCheck")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidUserCheck")]
/// *This icon requires the feature* `BiSolidUserCheck` *to be enabled*.
#[component]
pub fn UserCheck(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M8 12.052c1.995 0 3.5-1.505 3.5-3.5s-1.505-3.5-3.5-3.5-3.5 1.505-3.5 3.5 1.505 3.5 3.5 3.5zM9 13H7c-2.757 0-5 2.243-5 5v1h12v-1c0-2.757-2.243-5-5-5zm11.294-4.708-4.3 4.292-1.292-1.292-1.414 1.414 2.706 2.704 5.712-5.702z" /></svg>
   }
}