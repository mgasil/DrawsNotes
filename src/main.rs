#![allow(non_snake_case, unused)]
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use serde::Deserialize;
#[cfg(feature = "ssr")]
use surrealdb::engine::remote::ws::Client;
#[cfg(feature = "ssr")]
use surrealdb::sql::Thing;
#[cfg(feature = "ssr")]
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

use std::{
    collections::{HashMap, HashSet},
    sync::{RwLock, RwLockWriteGuard},
    time::Duration,
};

use crate::component::{notebook_bar::NotebookBar, notes_bar::NotesBar, notes_view::NotesView};
use crate::model::notebook::NotebookNoteCount;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::{server_fn::error::ServerFnErrorErr, *};
use log::LevelFilter;
use model::{note::Note, notebook::Notebook};
pub mod component;
pub mod model;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    LaunchBuilder::new(app).launch();
}

const NOTE_TABLE: &str = "note";
const NOTEBOOK_TABLE: &str = "notebook";

#[cfg(feature = "ssr")]
lazy_static! {
    static ref DB: AsyncOnce<Surreal<Client>> = {
        AsyncOnce::new(async {
            log::info!("connect surrealdb client");
            let db: Surreal<Client> = Surreal::new::<Ws>("127.0.0.1:8000")
                .await
                .expect("couldn't connect to surrealdb");

            log::info!("use ns");
            db.use_ns("test")
                .use_db("test")
                .await
                .expect("could not use ns and db");

            db
        })
    };
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[server]
async fn upsert_note(note: Note) -> Result<String, ServerFnError> {
    log::info!("upserting note {:?}", &note);
    let con = DB.get().await;

    let res: Vec<Record> = if let Some(id) = note.id {
        con.query("UPDATE ONLY type::thing($id) SET notebook = type::thing($notebook), title = $title, content = $content;")
        .bind(("id", id))
        .bind(("notebook", note.notebook))
        .bind(("content", note.content))
        .bind(("title", note.title))
        .await
        .expect("issue on await")
        .take(0)
        .expect("issue on take")
    } else {
        con.query("CREATE note SET notebook = type::thing($notebook), title = $title, content = $content;")
        .bind(("notebook", note.notebook))
        .bind(("content", note.content))
        .bind(("title", note.title))
        .await
        .expect("issue on await")
        .take(0)
        .expect("issue on take")
    };

    match res.first() {
        Some(Record { id }) => Ok(id.to_string()),
        _ => Err(ServerFnError::ServerError("couldnt get id".to_string())),
    }
}

#[server]
async fn upsert_notebook(notebook: Notebook) -> Result<String, ServerFnError> {
    log::info!("upserting notebook {:?}", &notebook);
    let con = DB.get().await;

    let res: Vec<Record> = if let Some(id) = notebook.id {
        con.query("UPDATE ONLY type::thing($id) SET name = $name")
            .bind(("name", notebook.name))
            .await
            .expect("issue on await")
            .take(0)
            .expect("issue on take")
    } else {
        con.query("CREATE notebook SET name = $name;")
            .bind(("name", notebook.name))
            .await
            .expect("issue on await")
            .take(0)
            .expect("issue on take")
    };

    match res.first() {
        Some(Record { id }) => Ok(id.to_string()),
        _ => Err(ServerFnError::ServerError("couldnt get id".to_string())),
    }
}

#[server]
async fn delete_notebook(notebook: Notebook) -> Result<(), ServerFnError> {
    // {
    //     let mut notebooks = NOTEBOOKS.write()?;
    //     if !notebooks.remove(&notebook) {
    //         return Err(ServerFnError::Request("Notebook not found".to_string()));
    //     }
    // }

    // {
    //     let mut notes = NOTES.write()?;
    //     notes
    //         .remove(&notebook.id)
    //         .ok_or(ServerFnError::ServerError("note found".to_string()))?;
    // }

    Ok(())
}

#[server]
async fn get_notebooks() -> Result<Vec<Notebook>, ServerFnError> {
    // do we still need this ? this is to get around a dioxus bug
    // tokio::time::sleep(Duration::from_millis(1000));
    log::info!("get notebooks");
    let con = DB.get().await;

    // really don't want this to be two queries, but this seemed like the lesser of evils
    let mut res: surrealdb::Response = con
        .query("SELECT type::string(id) as id, type::string(name) as name FROM type::table($table)")
        .bind(("table", NOTEBOOK_TABLE))
        .await
        .expect("issue on await");

    let mut res: Result<Vec<Notebook>, _> = res.take(0);

    match res {
        Ok(mut notebooks) => {
            //now grab the counts
            let mut counts: surrealdb::Response = con
                .query("SELECT type::string(notebook) as id, count(id) as count FROM type::table($table) GROUP BY id")
                .bind(("table", NOTE_TABLE))
                .await
                .expect("issue on await");

            let counts: Result<Vec<NotebookNoteCount>, _> = counts.take(0);
            match counts {
                Ok(counts) => {
                    //turn the notebooks into a map from id -> Notebook
                    let count_map: HashMap<String, NotebookNoteCount> = counts
                        .into_iter()
                        .map(|notebook| (notebook.id.clone(), notebook))
                        .collect();

                    notebooks.iter_mut().for_each(|notebook| {
                        let id = notebook.id.as_ref();
                        if let Some(id) = id {
                            let ct: Option<&NotebookNoteCount> = count_map.get(id);
                            notebook.count = Some(ct.map(|nbct| nbct.count).unwrap_or(0));
                        } else {
                            notebook.count = Some(0);
                        }
                    });

                    Ok(notebooks)
                }
                Err(e) => {
                    log::error!("issue getting note counts {:?}", e);
                    Err(e.into())
                }
            }
        }
        Err(e) => {
            log::error!("error getting notebooks {:?}", e);
            Err(e.into())
        }
    }
}

#[server]
async fn get_note_summaries(notebook_id: Option<String>) -> Result<Vec<Note>, ServerFnError> {
    // do we still need this ? this is to get around a dioxus bug
    // tokio::time::sleep(Duration::from_millis(200));

    log::info!("getting summaries for notebook {:?}", notebook_id);
    use std::str::FromStr;
    let con = DB.get().await;

    // probably a way to make this more concise
    let res: Vec<Note> = if let Some(notebook_id) = notebook_id {
        let notebook_thing = Thing::from_str(&notebook_id)
            .map_err(|_| ServerFnError::ServerError("error making thing".to_string()))?;
        con
        .query("SELECT type::string(id) as id, title, string::slice(content, 0, 40) as content, type::string(notebook) as notebook FROM type::table($table) WHERE notebook=type::thing($notebook_thing);")
        .bind(("table", NOTE_TABLE))
        .bind(("notebook_thing", notebook_thing))
        .await
        .expect("issue on await")
        .take(0)
        .expect("issue on take")
    } else {
        con
        .query("SELECT type::string(id) as id, title, string::slice(content, 0, 40) as content, type::string(notebook) as notebook FROM type::table($table);")
        .bind(("table", NOTE_TABLE))
        .await
        .expect("issue on await")
        .take(0)
        .expect("issue on take")
    };

    log::info!("summaries from db {:?}", &res);
    let res: Vec<Note> = res.into_iter().map(|notedb| notedb.into()).collect();

    log::info!("summaries converted {:?}", &res);
    Ok(res)
}

#[server]
async fn get_note(notebook_id: String, note_id: String) -> Result<Note, ServerFnError> {
    let con = DB.get().await;
    let res: Option<Note> = con
        .query("SELECT * FROM $table WHERE notebook=$notebook_id AND id=$note_id")
        .bind(("table", NOTE_TABLE))
        .bind(("notebook_id", &notebook_id))
        .bind(("note_id", &note_id))
        .await?
        .take(0)?;

    res.ok_or(ServerFnError::ServerError("couldn't get note".to_string()))
}

#[server]
async fn delete_note(note_id: String) -> Result<(), ServerFnError> {
    let con = DB.get().await;

    log::info!("note id: {:?}", &note_id);

    let res = con
        .query("DELETE type::thing($note_id)")
        .bind(("note_id", note_id))
        .await?;

    log::info!("delete response: {:?}", res);
    Ok(())
}

fn app(cx: Scope) -> Element {
    let notebooks: &UseFuture<Result<Vec<Notebook>, ServerFnError>> =
        use_future(cx, (), |_| get_notebooks());
    let mut selected_notebook: &UseState<Option<Notebook>> = use_state(cx, || None);
    let mut selected_note = use_state(cx, || None);
    let mut note_summaries: &UseFuture<Result<Vec<Note>, ServerFnError>> =
        use_future(cx, (selected_notebook), |selected_notebook| async move {
            if let Some(Notebook { id: id, .. }) = selected_notebook.current().as_ref() {
                get_note_summaries(id.clone()).await
            } else {
                Ok(vec![])
            }
        });

    use_effect(cx, (selected_notebook,), |(selected_notebook,)| {
        to_owned!(selected_note);
        log::info!("selected_notebook effect!!!!!");
        async move {
            selected_note.set(None);
            log::info!("selected note set to None");
        }
    });

    match notebooks.state() {
        UseFutureState::Complete(_) => {
            render! {
                div {
                    class: "flex h-screen text-white",
                    NotebookBar {
                        notebooks: notebooks,
                        selected_notebook: selected_notebook.clone(),
                    },
                    if let Some(selected_notebook) = selected_notebook.current().as_ref() {
                        rsx! {
                            NotesBar {
                                note_summaries: note_summaries,
                                notebooks: notebooks,
                                selected_note: selected_note,
                                selected_notebook: selected_notebook.clone(),
                            },
                            NotesView {
                                notebooks: notebooks,
                                selected_note: selected_note.clone(),
                                note_summaries: note_summaries,
                            }
                        }
                    } else {
                        rsx! {
                            div {
                                class: "h-full w-full bg-gray-800 flex items-center justify-center p-8 gap-4 text-gray-400 text-lg",
                                div {
                                    class: "flex flex-row items-center",
                                    svg {
                                        class: "shrink h-4 px-2",
                                        xmlns:"http://www.w3.org/2000/svg",
                                        // these colors are the same as text-gray-400
                                        stroke: "rgb(156 163 175 / var(--tw-text-opacity))",
                                        fill: "rgb(156 163 175 / var(--tw-text-opacity))",
                                        view_box: "0 0 512 512",
                                        path {
                                            d: "M512 256A256 256 0 1 0 0 256a256 256 0 1 0 512 0zM231 127c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9l-71 71L376 232c13.3 0 24 10.7 24 24s-10.7 24-24 24l-182.1 0 71 71c9.4 9.4 9.4 24.6 0 33.9s-24.6 9.4-33.9 0L119 273c-9.4-9.4-9.4-24.6 0-33.9L231 127z",
                                        }
                                    },
                                    div {
                                        "Select a notebook"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        _ => {
            render! {
                div {
                    class: "h-screen w-screen bg-gray-800 flex items-center justify-center p-8 gap-4 text-gray-400 text-lg",
                    div {
                        class: "flex flex-row h-8 items-center",
                        svg {
                            class: "spinner shrink h-4 px-2",
                            xmlns: "http://www.w3.org/2000/svg",
                            stroke: "rgb(156 163 175 / var(--tw-text-opacity))",
                            fill: "rgb(156 163 175 / var(--tw-text-opacity))",
                            view_box: "0 0 512 512",
                            path {
                                d: "M304 48a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zm0 416a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zM48 304a48 48 0 1 0 0-96 48 48 0 1 0 0 96zm464-48a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zM142.9 437A48 48 0 1 0 75 369.1 48 48 0 1 0 142.9 437zm0-294.2A48 48 0 1 0 75 75a48 48 0 1 0 67.9 67.9zM369.1 437A48 48 0 1 0 437 369.1 48 48 0 1 0 369.1 437z"
                            }
                        },
                        div {
                            "Loading",
                        }
                    }
                }
            }
        }
    }
}
