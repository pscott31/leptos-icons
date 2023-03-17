#[cfg(feature = "ImAddressBook")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImAddressBook")]
/// *This icon requires the feature* `ImAddressBook` *to be enabled*.
#[component]
pub fn AddressBook(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M3 0v16h12v-16h-12zM9 4.005c1.102 0 1.995 0.893 1.995 1.995s-0.893 1.995-1.995 1.995-1.995-0.893-1.995-1.995 0.893-1.995 1.995-1.995v0zM12 12h-6v-1c0-1.105 0.895-2 2-2v0h2c1.105 0 2 0.895 2 2v1z" /><path fill="#000000" d="M1 1h1.5v3h-1.5v-3z" /><path fill="#000000" d="M1 5h1.5v3h-1.5v-3z" /><path fill="#000000" d="M1 9h1.5v3h-1.5v-3z" /><path fill="#000000" d="M1 13h1.5v3h-1.5v-3z" /></svg>
   }
}