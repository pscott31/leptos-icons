#[cfg(feature = "SiKaufland")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiKaufland")]
/// *This icon requires the feature* `SiKaufland` *to be enabled*.
#[component]
pub fn Kaufland(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 24h24V0H0zm23.008-.989H.989V.989h22.022zM3.773 3.776h7.651v7.65H3.773zm8.801 0v7.652l7.653-7.652zm-8.801 8.8h7.651v7.651H3.773zm8.801-.004v7.652h7.653z" /></svg>
   }
}