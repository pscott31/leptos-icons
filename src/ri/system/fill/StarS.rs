#[cfg(feature = "RiSystemFillStarS")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillStarS")]
/// *This icon requires the feature* `RiSystemFillStarS` *to be enabled*.
#[component]
pub fn StarS(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 17l-5.878 3.59 1.598-6.7-5.23-4.48 6.865-.55L12 2.5l2.645 6.36 6.866.55-5.231 4.48 1.598 6.7z" /></g></svg>
   }
}