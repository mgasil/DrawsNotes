use crate::api::{DeleteNote, UpsertNote};
use crate::model::{count::CountEvent, note::Note, notebook::Notebook};
use crate::UpsertNotebook;
use leptos::{Action, ReadSignal, ServerFnError, WriteSignal};

pub type NotebookSignal = (ReadSignal<Option<Notebook>>, WriteSignal<Option<Notebook>>);
pub type NoteSignal = (ReadSignal<Option<Note>>, WriteSignal<Option<Note>>);
pub type CountSignal = (
    ReadSignal<Option<CountEvent>>,
    WriteSignal<Option<CountEvent>>,
);

pub type UpsertNoteAction = Action<UpsertNote, Result<String, ServerFnError>>;
pub type DeleteNoteAction = Action<DeleteNote, Result<(), ServerFnError>>;
pub type UpsertNotebookAction = Action<UpsertNotebook, Result<String, ServerFnError>>;
