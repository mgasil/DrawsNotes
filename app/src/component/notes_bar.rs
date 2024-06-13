use crate::component::icon::*;
use crate::error_template::ErrorTemplate;
use crate::types::*;
use crate::{
    get_note_summaries,
    model::{count::*, note::Note, notebook::Notebook},
    UpsertNote,
};
use leptos::*;

#[component]
fn NoteItem(note: Note) -> impl IntoView {
    const SELECTED_NOTE_STYLE: &str =
        "w-full flex flex-col pl-2 bg-gray-900 border-t border-gray-600 select-none w-64 max-w-64";
    const UNSELECTED_NOTE_STYLE: &str =
        "w-full flex flex-col pl-2 border-t border-gray-600 select-none w-64 max-w-64";

    let (selected_note, set_selected_note) = expect_context::<NoteSignal>();

    let select_note = {
        let note = note.clone();
        move |_| set_selected_note(Some(note.clone()))
    };

    let selection_style = move || {
        if let Some(selected) = selected_note() {
            if selected.id == note.id {
                SELECTED_NOTE_STYLE
            } else {
                UNSELECTED_NOTE_STYLE
            }
        } else {
            UNSELECTED_NOTE_STYLE
        }
    };

    view! {
        <div class={selection_style} on:click=select_note>
            <div>{note.title}</div>
            <div class="text-gray-400 text-nowrap truncate">{note.content}</div>
        </div>
    }
}

#[component]
fn NoteList(list: Vec<Note>) -> impl IntoView {
    view! {
        <For
             each=move || list.clone()
             key=|state| state.clone()
             let:note
        >
            <NoteItem note=note>

            </NoteItem>
         </For>
    }
}

#[component]
pub fn NotesBar(add_note: UpsertNoteAction, delete_note: DeleteNoteAction) -> impl IntoView {
    let (_, set_count_event) = expect_context::<CountSignal>();
    let (selected_notebook, _) = expect_context::<NotebookSignal>();

    let note_summaries = create_resource(
        move || {
            (
                selected_notebook(),
                add_note.version().get(),
                delete_note.version().get(),
            )
        },
        move |_| async move {
            if let Some(Notebook { id, .. }) = selected_notebook.get_untracked() {
                get_note_summaries(id).await
            } else {
                Ok(vec![])
            }
        },
    );

    let note_list = move || {
        note_summaries.get().map(move |summaries| match summaries {
            Ok(summaries) => view! {
                <NoteList
                    list=summaries
                />
            }
            .into_view(),
            Err(err) => view! {
                <pre class="error">"Server Error: " {err.to_string()}</pre>

            }
            .into_view(),
        })
    };

    let not_selected = move || {
        if let Some(notebook) = selected_notebook() {
            if notebook == Notebook::all() {
                return false;
            }
        }
        true
    };

    let add_note = move |_| {
        if let Some(notebook_id) = selected_notebook.get_untracked().unwrap().id {
            add_note.dispatch(UpsertNote {
                note: Note::new(notebook_id.clone()),
            });

            set_count_event(Some(CountEvent {
                id: notebook_id,
                action: CountAction::Increase,
            }));
        }
    };

    view! {
        <div class="w-64 flex-shrink-0 h-full overflow-y-auto bg-gray-700 cursor-default">
            <div class="flex flex-row items-center">
                <div class="text-xl grow flex flex-row justify-center">
                    { move || {selected_notebook().unwrap().name.clone()}}
                </div>
                <Show
                        when=not_selected
                    >
                    <Icon
                        r#type=AddNote
                        on:click=add_note
                    />
                </Show>
            </div>
            <Transition
                fallback=move || view! { <div>"Loading..."</div> }
            >
                <ErrorBoundary fallback=|errors| view!{<ErrorTemplate errors=errors/>}>
                    {note_list}
                </ErrorBoundary>
            </Transition>
        </div>
    }
}
