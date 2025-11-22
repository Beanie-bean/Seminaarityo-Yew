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
}

struct App {
    todos: Vec<TodoItem>,
    focus_ref: NodeRef,
}

enum Msg {
    Add(String),
    Remove(usize)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let todos = LocalStorage::get(KEY).unwrap_or_else(|_| Vec::new());
        let focus_ref = NodeRef::default();
        Self { todos, focus_ref }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <h2>{ "Todos" }</h2>
                { self.view_input(ctx.link()) }
            <ul>
              { for self
                .todos
                .iter()
                .enumerate()
                .map(|(i, e)| self.view_todo((i, e), ctx.link()))
                }
            </ul>
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
                };
                self.todos.push(todo);
            }
            Msg::Remove(i)=> {
                self.todos.remove(i);
            }
        }
        LocalStorage::set(KEY, &self.todos).expect("failed to set");
        true
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
                }
                else {
                    None
                }
            } else {
                None
            }
        });
        
        html! {
            <div>
                <input
                    type="text"
                    {onkeypress}
                />
            </div>
        }
    }
    fn view_todo(&self, (i, todo): (usize, &TodoItem), link: &Scope<Self>) -> Html {
        html! {
            <li>
                <div>
                    <label>{&todo.text}</label>
                    <button onclick={link.callback(move |_| Msg::Remove((i)))}>
                        {"Remove"}
                    </button>
                </div>
            </li>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
