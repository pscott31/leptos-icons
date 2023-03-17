#[cfg(feature = "IoBluetoothSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBluetoothSharp")]
/// *This icon requires the feature* `IoBluetoothSharp` *to be enabled*.
#[component]
pub fn BluetoothSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M397.41,161.13,236-.28v212.8L141.83,131.8l-26,30.37L225.27,256,115.8,349.83l26,30.37L236,299.48v212.8L397.41,350.87,286.73,256ZM276,96.28l62.59,62.59L276,212.52Zm62.58,256.85L276,415.72V299.48Z" /></svg>
   }
}