#[cfg(feature = "RiSystemFillArrowRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillArrowRight")]
/// *This icon requires the feature* `RiSystemFillArrowRight` *to be enabled*.
#[component]
pub fn ArrowRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 13H4v-2h8V4l8 8-8 8z" /></g></svg>
   }
}