use crate::model::count::*;
use crate::types::CountSignal;
use leptos::*;

#[component]
pub fn Counter(initial_count: u32, notebook_id: Option<String>) -> impl IntoView {
    let (count_event, _) = expect_context::<CountSignal>();

    let (count, set_count) = create_signal(initial_count);

    let updated_count = move || {
        if let Some(count_event) = count_event() {
            if let Some(notebook_id) = notebook_id.clone() {
                if notebook_id == count_event.id {
                    match count_event.action {
                        CountAction::Increase => set_count.update(|value| *value += 1),
                        CountAction::Decrease => set_count.update(|value| *value -= 1),
                        CountAction::Reset => set_count(0),
                    }
                }
            } else {
                match count_event.action {
                    CountAction::Increase => set_count.update(|value| *value += 1),
                    CountAction::Decrease => set_count.update(|value| *value -= 1),
                    CountAction::Reset => set_count(0),
                }
            }
        }

        count.get_untracked()
    };

    view! {
        <div class="pr-2 flex items-center shrink">
            <div class="rounded-full bg-gray-700 text-xs min-w-[20px] h-[20px] flex items-center justify-center">
                { updated_count }
            </div>
        </div>
    }
}
