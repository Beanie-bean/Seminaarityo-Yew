use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::{html::Scope, prelude::*};

const KEY: &str = "yew.todo";

#[derive(Properties, PartialEq, Clone, Deserialize, Serialize)]
struct TodoItem {
    text: String,
    #[prop_or(false)]
    complete: bool,
    editing: bool,
}

struct App {
    state: State,
    focus_ref: NodeRef,
}

struct State {
    todos: Vec<TodoItem>,
    edit_value: String,
}

enum Msg {
    Add(String),
    Remove(usize),
    Edit(usize, String),
    ToggleEdit(usize),
    Toggle(usize),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let todos = LocalStorage::get(KEY).unwrap_or_else(|_| Vec::new());
        let state = State {
            todos: todos,
            edit_value: String::new(),
        };
        let focus_ref = NodeRef::default();
        Self { state, focus_ref }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hidden_class = if self.state.todos.is_empty() {
            "hidden"
        } else {
            ""
        };
        html! {
        <div>
            <section class="app">
                <header>
                    <h2>{ "Todos" }</h2>
                    { self.view_input(ctx.link()) }
                </header>
                    <ul class="list">
                        { for self
                            .state
                            .todos
                            .iter()
                            .enumerate()
                            .map(|(i, e)| self.view_todo((i, e), ctx.link()))
                            }
                    </ul>
                    <p class={classes!("edithelp", hidden_class)}>{"Double click to edit"}</p>
            </section>
        </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add(text) => {
                let text = text.trim();

                let todo = TodoItem {
                    text: text.to_string(),
                    complete: false,
                    editing: false,
                };
                self.state.todos.push(todo);
            }
            Msg::Remove(i) => {
                self.state.todos.remove(i);
            }
            Msg::Edit(i, edit_value) => {
                self.state.complete_edit(i, edit_value.trim().to_string());
                self.state.edit_value = "".to_string();
            }
            Msg::ToggleEdit(i) => {
                let todo = self.state.todos.iter().nth(i).unwrap();
                self.state.edit_value.clone_from(&todo.text);
                self.state.toggle_edit(i);
            }
            Msg::Toggle(i) => {
                self.state.toggle_completed(i);
            }
        }
        LocalStorage::set(KEY, &self.state.todos).expect("failed to set");
        true
    }
}

impl State {
    fn toggle_edit(&mut self, i: usize) {
        let todo = self.todos.get_mut(i).unwrap();
        todo.editing = !todo.editing;
    }

    fn toggle_completed(&mut self, i: usize) {
        let todo = self.todos.get_mut(i).unwrap();
        todo.complete = !todo.complete
    }

    fn complete_edit(&mut self, i: usize, value: String) {
        if value.is_empty() {
            self.remove(i);
        } else {
            let todo = self.todos.get_mut(i).unwrap();
            todo.text = value;
            todo.editing = !todo.editing;
        }
    }
    fn remove(&mut self, i: usize) {
        self.todos.remove(i);
    }
}

impl App {
    fn view_input(&self, link: &Scope<Self>) -> Html {
        let onkeypress = link.batch_callback(|event: KeyboardEvent| {
            if event.key() == "Enter" {
                let input: HtmlInputElement = event.target_unchecked_into();
                let value = input.value();
                if value != "" {
                    input.set_value("");
                    Some(Msg::Add(value))
                } else {
                    None
                }
            } else {
                None
            }
        });
    
        html! {
            <div>
                <div class="todoinput">
                    <input
                        class="inputfield"
                        placeholder="Add todo"
                        type="text"
                        {onkeypress}
                    />
                </div>
                <p class="help">{"Press enter to submit"}</p>
            </div>
        }
    }

    fn view_todo(&self, (i, todo): (usize, &TodoItem), link: &Scope<Self>) -> Html {
        let mut class = Classes::from("todo");
        if todo.editing {
            class.push("editing");
        }
        if todo.complete {
            class.push("completed");
        }
        html! {
            <div class="todolistbox">
                <li {class}>
                    <div class="todoline">
                        <input
                            class="checkbox"
                            type="checkbox"
                            checked={todo.complete}
                            onclick={link.callback(move |_| Msg::Toggle(i))}
                        />
                        <label class="text" ondblclick={link.callback(move |_| Msg::ToggleEdit(i))}>{&todo.text}</label>
                        <button class="remove" onclick={link.callback(move |_| Msg::Remove(i))}></button>
                    </div>
                    {self.view_todo_edit_input((i, todo), link)}
                </li>
            </div>
        }
    }

    fn view_todo_edit_input(&self, (i, todo): (usize, &TodoItem), link: &Scope<Self>) -> Html {
        let edit = move |input: HtmlInputElement| {
            let value = input.value();
            input.set_value("");
            Msg::Edit(i, value)
        };

        // Next three lines of code are taken directly from https://github.com/yewstack/yew/blob/master/examples/todomvc/src/main.rs
        let onkeypress = link.batch_callback(move |e: KeyboardEvent| {
            (e.key() == "Enter").then(|| edit(e.target_unchecked_into()))
        });

        if todo.editing {
            html! {
                <div class="edit">
                    <input
                        class="editfield"
                        type="text"
                        ref={self.focus_ref.clone()}
                        value={self.state.edit_value.clone()}
                        {onkeypress}
                    />
                </div>
            }
        } else {
            html! { <input type="hidden" /> }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
