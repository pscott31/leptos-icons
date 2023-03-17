#[cfg(feature = "BiSolidCalendarAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCalendarAlt")]
/// *This icon requires the feature* `BiSolidCalendarAlt` *to be enabled*.
#[component]
pub fn CalendarAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 22h14c1.103 0 2-.897 2-2V6c0-1.103-.897-2-2-2h-2V2h-2v2H9V2H7v2H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2zM5 7h14v2H5V7z" /></svg>
   }
}