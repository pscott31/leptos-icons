#[cfg(feature = "VsSymbolSnippet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSymbolSnippet")]
/// *This icon requires the feature* `VsSymbolSnippet` *to be enabled*.
#[component]
pub fn SymbolSnippet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M2.5 1l-.5.5V13h1V2h11v11h1V1.5l-.5-.5h-12zM2 15v-1h1v1H2zm3-1H4v1h1v-1zm1 0h1v1H6v-1zm3 0H8v1h1v-1zm1 0h1v1h-1v-1zm5 1v-1h-1v1h1zm-3-1h1v1h-1v-1z" /></svg>
   }
}