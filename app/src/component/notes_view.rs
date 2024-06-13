use crate::error_template::ErrorTemplate;
use crate::types::*;
use crate::{
    component::{icon::*, loading::Loading},
    get_note,
    model::{count::*, note::Note},
    DeleteNote, UpsertNote,
};
use leptos::*;

#[component]
pub fn NotesView(update_note: UpsertNoteAction, delete_note: DeleteNoteAction) -> impl IntoView {
    let (selected_note, set_selected_note) = expect_context::<NoteSignal>();
    let (_, set_count_event) = expect_context::<CountSignal>();

    let (new_title, set_new_title) = create_signal(String::new());
    let (new_content, set_new_content) = create_signal(String::new());

    let full_note = create_resource(
        move || (selected_note()),
        move |_| async move {
            if let Some(Note { id: Some(id), .. }) = selected_note.get_untracked() {
                let note_res = get_note(id.clone()).await;
                if let Ok(note) = note_res.clone() {
                    set_new_content(note.content);
                    set_new_title(note.title);
                }
                Some(note_res)
            } else {
                None
            }
        },
    );

    let form_controls = move |note: Note| {
        let Note {
            id,
            title,
            content,
            notebook,
        } = note.clone();

        view! {
            <input
            placeholder="Title"
            class="w-full p-2 bg-gray-700 border border-gray-600 rounded-md shrink focus:outline-none focus:ring-0"
            prop:value=new_title
            on:input=move |ev| set_new_title(event_target_value(&ev))
            />
            <textarea
                placeholder="Content"
                class="w-full p-2 bg-gray-700 border border-gray-600 rounded-md resize-none grow focus:outline-none focus:ring-0"
                prop:value=new_content
                on:input=move |ev| set_new_content(event_target_value(&ev))
            />

            <button
                class="px-4 py-2 bg-blue-500 hover:bg-blue-600 rounded-md shrink disabled:bg-neutral-600"
                disabled=move || {(title == new_title() && content == new_content()) || new_title.get_untracked().is_empty()}
                on:click=move |_| {
                    let new_content = new_content();
                    let new_title = new_title();

                    update_note.dispatch(UpsertNote { note: Note {
                        id: id.clone(),

                        title: new_title,
                        content: new_content,
                        notebook: notebook.clone(),
                    }});
                }
            >
                "Save"
            </button>
            <button
                class="text-red-500"
                on:click=move |_| {
                    let note = note.clone();
                    if let Some(id) = note.id {

                        let _res = delete_note.dispatch(DeleteNote { note_id: id }); // delete_note(id).await;
                        //if let () = res {
                        set_count_event(Some(CountEvent { id: note.notebook, action: CountAction::Decrease }));
                        set_selected_note(None);
                        // } else {
                        //     log::error!("error deleting note");
                        // }
                    }
                }
            >
                "Delete"
            </button>
        }
    };

    view! {

            <Show
                when=move || selected_note().is_some()
                fallback=move || view! {
                    <div class="h-full w-full bg-gray-800 flex items-center justify-center p-8 gap-4 text-gray-400 text-lg">
                    <div class="flex flex-row items-center">
                        <Icon r#type=Unselected />
                        <div>"Select a note"</div>
                    </div>
                    </div>
                }
            >
                <div class="h-full bg-gray-800 flex flex-col items-center justify-center p-8 gap-4 text-white grow">

                    <Transition
                        fallback=move || view! { <Loading fullscreen=false /> }
                    >
                        <ErrorBoundary fallback=|errors| view!{<ErrorTemplate errors=errors/>}>
                            {move || full_note.get()
                                .map(move |note| match note {
                                    Some(Ok(note)) => {
                                        view! {
                                            {form_controls(note)}
                                        }.into_view()
                                    },
                                    Some(Err(err)) => {
                                        view! {
                                            <pre class="error">"Server Error: " {err.to_string()}</pre>
                                        }.into_view()
                                    },
                                    None => {
                                        view! {


                                        }.into_view()
                                    }
                                }

                            )}
                        </ErrorBoundary>
                    </Transition>
                </div>
            </Show>

    }
}
