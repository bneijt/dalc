extern crate chrono;
use chrono::prelude::*;
use leptos::*;


fn date_information(date: NaiveDate) -> String {
    format!(
        "Naive date, {}
        week of the year: {}
        day of the year: {}
        ",
        date.format("%F: %A %B %e"),
        date.format("%U"),
        date.format("%j"),
    )
}

fn parse_date(input_value: String) -> String {
    let failure_response = String::from("no banana");
    match NaiveDate::parse_from_str(&input_value, "%Y-%m-%d") {
        Ok(date) => date_information(date),
        Err(_) => failure_response,
    }
    //take the first line
    //If it contains a timestamp, parse the timestamp and return the iso text representation
}

#[component]
fn ControlledComponent(cx: Scope) -> impl IntoView {
    // create a signal to hold the value
    let (name, set_name) = create_signal(cx, "Controlled".to_string());

    view! { cx,
        <textarea 
            rows=5
            autofocus=true
            on:input=move |ev| {
                set_name(parse_date(event_target_value(&ev)));
            }
            // prop:value=name
        />
        <pre>"Name is: " {name}</pre>
    }
}

#[component]
pub fn SimpleCounter(cx: Scope, initial_value: i32) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = create_signal(cx, initial_value);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    // create user interfaces with the declarative `view!` macro
    view! { cx,
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {value} "!"</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Controlled Component"</h2>
        <ControlledComponent/>
        <h2>"Uncontrolled Component"</h2>
        <SimpleCounter initial_value=3 />
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}