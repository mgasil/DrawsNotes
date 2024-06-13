//use std::time::Duration;

use crate::component::{
    counter::Counter,
    icon::{Icon, IconType},
    loading::*,
};
use crate::error_template::ErrorTemplate;
use crate::types::*;
use crate::utility::set_focus;
use crate::{get_notebooks, model::notebook::Notebook, UpsertNotebook};
use ev::{KeyboardEvent, MouseEvent};
use leptos::*;

#[component]
fn NotebookItem(notebook: Notebook) -> impl IntoView {
    const SELECTED_NOTEBOOK_STYLE: &str =
        "flex flex-row content-center items-center bg-gray-900 pl-4";
    const UNSELECTED_NOTEBOOK_STYLE: &str = "flex flex-row content-center items-center pl-4";

    let (selected_notebook, set_selected_notebook) = expect_context::<NotebookSignal>();
    let (_, set_selected_note) = expect_context::<NoteSignal>();

    let select_notebook = {
        let notebook = notebook.clone();
        move |ev: MouseEvent| {
            ev.prevent_default();
            if let Some(selected) = selected_notebook.get_untracked() {
                if selected.id != notebook.id {
                    set_selected_notebook(Some(notebook.clone()));
                    set_selected_note(None);
                }
            } else {
                set_selected_notebook(Some(notebook.clone()));
            }
        }
    };

    let selection_style = {
        let notebook_id = notebook.id.clone();
        move || {
            if let Some(selected) = selected_notebook() {
                if selected.id == notebook_id {
                    SELECTED_NOTEBOOK_STYLE
                } else {
                    UNSELECTED_NOTEBOOK_STYLE
                }
            } else {
                UNSELECTED_NOTEBOOK_STYLE
            }
        }
    };

    view! {
        <div class={selection_style}
             on:click=select_notebook
        >
            <div class="grow">{notebook.name}</div>
            <Counter
                initial_count=notebook.count.unwrap_or(0)
                notebook_id=notebook.id
            />
        </div>
    }
}

#[component]
pub fn NotebookList(notebooks: Vec<Notebook>) -> impl IntoView {
    //let (rename, set_rename) = create_signal(String::new());

    view! {
       <For
            each=move || notebooks.clone()
            key=|state| state.clone()
            let:notebook
        >
            <NotebookItem notebook=notebook/>
        </For>
    }
}

#[component]
pub fn NotebookBar(upsert_notebook: UpsertNotebookAction) -> impl IntoView {
    const SELECTED_ALL_STYLE: &str = "flex flex-row content-center items-center bg-gray-900";
    const UNSELECTED_ALL_STYLE: &str = "flex flex-row content-center items-center";

    let (selected_notebook, set_selected_notebook) = expect_context::<NotebookSignal>();
    let (_, set_selected_note) = expect_context::<NoteSignal>();
    let (_, set_count_event) = expect_context::<CountSignal>();

    let (creating_notebook, set_creating_notebook) = create_signal(false);
    let input_element: NodeRef<html::Input> = create_node_ref();

    let notebooks = create_resource(
        move || upsert_notebook.version().get(),
        |_| async move { get_notebooks().await },
    );

    create_effect(move |_| {
        if creating_notebook.get() {
            set_focus(input_element);
        }
    });

    let submit_notebook = move |_ev: KeyboardEvent| {
        if let Some(input) = input_element.get() {
            upsert_notebook.dispatch(UpsertNotebook {
                notebook: Notebook {
                    id: None,
                    name: input.value(),
                    count: None,
                },
            });

            set_count_event(None);
            set_creating_notebook(false);
        }
    };

    let keyboard_handler = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            submit_notebook(ev);
        } else if ev.key() == "Escape" {
            set_creating_notebook(false);
        }
    };

    let select_notebook = move |ev: MouseEvent| {
        ev.prevent_default();
        if let Some(selected) = selected_notebook() {
            if selected.id != Notebook::all().id {
                set_selected_notebook(Some(Notebook::all()));
                set_selected_note(None);
            }
        } else {
            set_selected_notebook(Some(Notebook::all()));
        }
    };

    let create_notebooks_list = move || {
        notebooks.get().map(move |notebooks| match notebooks {
            Ok(notebooks) => view! {
                <NotebookList
                    notebooks=notebooks
                />
            }
            .into_view(),
            Err(err) => view! {
                <pre class="error">"Server Error: " {err.to_string()}</pre>

            }
            .into_view(),
        })
    };

    let selection_style = move || {
        if selected_notebook()
            .as_ref()
            .map(|n| n.id.is_none())
            .unwrap_or(false)
        {
            SELECTED_ALL_STYLE
        } else {
            UNSELECTED_ALL_STYLE
        }
    };

    view! {
                <div class="flex flex-col bg-gray-800 cursor-default">
                        <div class={selection_style}
                            on:click=select_notebook
                        >
                            <Icon r#type=IconType::Note />
                            <div class="grow">"All Notes"</div>
                            <Transition> {
                                notebooks
                                .get()
                                .map(|notebooks|
                                    notebooks
                                        .map(|nb|
                                            view! {
                                                <Counter
                                                    initial_count={nb.iter().map(|n| n.count.unwrap_or(0)).sum()}
                                                    notebook_id={None}
                                                />
                                            }
                                        )
                                        .unwrap_or_else(|_| view! { }.into_view())
        )
    }
                            </Transition>
                        </div>
                        <div class="flex flex-row flex-nowrap content-center items-center">
                            <Icon r#type=IconType::Notebook />
                            <div class="grow">"Notebooks"</div>
                            <Icon
                                r#type=IconType::AddNotebook
                                on:click=move |_| set_creating_notebook(true)
                            />
                        </div>
                        <div class="flex flex-col justify-h w-[200px] overflow-hidden">
                            <Transition
                                fallback=move || view! { <Loading fullscreen=true /> }
                            >
                                <ErrorBoundary fallback=|errors| view!{<ErrorTemplate errors=errors/>}>
                                    {create_notebooks_list}
                                </ErrorBoundary>
                            </Transition>
                            <Show
                                when=creating_notebook
                            >
                                <div class="py-1 px-2">
                                    <input
                                        class="w-full bg-gray-700 border border-gray-600 rounded-md shrink focus:outline-none focus:ring-0"
                                        node_ref=input_element
                                        on:keydown=keyboard_handler
                                    />
                                </div>
                            </Show>
                        </div>


                </div>

        }
}
