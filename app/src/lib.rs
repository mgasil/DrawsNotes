mod component;
pub mod api;
pub mod model;
pub mod types;
pub mod error_template;
pub mod utility;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::types::*;

use crate::error_template::{AppError, ErrorTemplate};

use crate::component::{
    notebook_bar::NotebookBar,
    notes_bar::NotesBar,
    notes_view::NotesView
};

use crate::api::*;
use crate::model::{
     notebook::Notebook,
     note::Note,
     count::CountEvent
};



#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_context(create_signal(None::<Notebook>));
    provide_context(create_signal(None::<Note>));
    provide_context(create_signal(None::<CountEvent>));

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    use crate::component::icon::{Icon, Unselected};
  
    let upsert_note: UpsertNoteAction = create_server_action::<UpsertNote>();
    let delete_note: DeleteNoteAction = create_server_action::<DeleteNote>();
    let upsert_notebook: UpsertNotebookAction = create_server_action::<UpsertNotebook>();

    let (selected_notebook, _) = expect_context::<NotebookSignal>();
    let not_empty = create_memo(move |_| selected_notebook().is_some());
    //let not_empty = is_some(selected_notebook);

    view! {  
        <div class="flex h-screen text-white">
            <NotebookBar
                upsert_notebook=upsert_notebook
            />
            <Show
                when=not_empty
                fallback=move || view! {
                    <div class="h-full w-full bg-gray-800 flex items-center justify-center p-8 gap-4 text-gray-400 text-lg">
                        <div class="flex flex-row items-center">
                            <Icon r#type=Unselected />
                            <div>"Select a notebook"</div>
                        </div>
                    </div>
                }
            >
                <NotesBar
                    add_note=upsert_note
                    delete_note=delete_note
                /> 
                <NotesView
                    update_note=upsert_note
                    delete_note=delete_note
                />
            </Show>   
        </div>
    }
}