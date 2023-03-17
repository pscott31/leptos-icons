#[cfg(feature = "RiDeviceLineDualSim1")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceLineDualSim1")]
/// *This icon requires the feature* `RiDeviceLineDualSim1` *to be enabled*.
#[component]
pub fn DualSim1(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path fill-rule="nonzero" d="M15 2l4.707 4.707a1 1 0 0 1 .293.707V21a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h10zm-.829 2H6v16h12V7.829L14.171 4zM13 16h-2v-6h-1V8h3v8z" /></g></svg>
   }
}