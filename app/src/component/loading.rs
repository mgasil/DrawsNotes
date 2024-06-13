use crate::component::icon::{Icon, IconType};
use leptos::*;

#[component]
pub fn Loading(fullscreen: bool) -> impl IntoView {
    let wh = if fullscreen {
        "h-screen w-screen"
    } else {
        "h-full w-full"
    };

    let base_css = "bg-gray-800 flex items-center justify-center p-8 gap-4 text-gray-400 text-lg";
    let css = format!("{} {}", base_css, wh);

    view! {
        <div class=css>
            <div class="flex flex-row h-8 items-center">
                <Icon
                    r#type=IconType::Loading
                />
                <div>
                    "Loading"
                </div>
            </div>
        </div>
    }
}
