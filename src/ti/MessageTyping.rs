#[cfg(feature = "TiMessageTyping")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiMessageTyping")]
/// *This icon requires the feature* `TiMessageTyping` *to be enabled*.
#[component]
pub fn MessageTyping(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M18 6h-13c-1.65 0-3 1.35-3 3v7c0 1.65 1.35 3 3 3h1v3l3-3h9c1.65 0 3-1.35 3-3v-7c0-1.65-1.35-3-3-3zm1 10c0 .542-.458 1-1 1h-13c-.542 0-1-.458-1-1v-7c0-.542.458-1 1-1h13c.542 0 1 .458 1 1v7zM7 14.5c-1.104 0-2-.896-2-2s.896-2 2-2 2 .896 2 2-.896 2-2 2zm0-3c-.552 0-1 .448-1 1s.448 1 1 1 1-.448 1-1-.448-1-1-1zM11.5 14.5c-1.104 0-2-.896-2-2s.896-2 2-2 2 .896 2 2-.896 2-2 2zm0-3c-.552 0-1 .448-1 1s.448 1 1 1 1-.448 1-1-.448-1-1-1zM16 14.5c-1.104 0-2-.896-2-2s.896-2 2-2 2 .896 2 2-.896 2-2 2zm0-3c-.552 0-1 .448-1 1s.448 1 1 1 1-.448 1-1-.448-1-1-1z" /></svg>
   }
}