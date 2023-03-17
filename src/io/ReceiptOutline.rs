#[cfg(feature = "IoReceiptOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoReceiptOutline")]
/// *This icon requires the feature* `IoReceiptOutline` *to be enabled*.
#[component]
pub fn ReceiptOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="160 336 160 48 192 64 224 48 255.94 64 288.31 48 320 64 351.79 48 383.72 64 416 48 448.01 64 480 48 480 272" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><path d="M480,272V384a80,80,0,0,1-80,80h0a80,80,0,0,1-80-80V336H48a15.86,15.86,0,0,0-16,16c0,64,6.74,112,80,112H400" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><line x1="224" y1="144" x2="416" y2="144" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="288" y1="224" x2="416" y2="224" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}