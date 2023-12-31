extern crate chrono;
use chrono::prelude::*;
use chrono::{DateTime, Utc};
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

///Try various date and datetime formats and return the first one that works
fn parse_input(input_value: String) -> Option<DateTime<Utc>> {
    print!("parse_input: {}", input_value);
    let parsers = [
        |ival: &str| {
            DateTime::parse_from_rfc3339(ival)
                .map(|x| x.with_timezone(&Utc))
                .ok()
        },
        |ival: &str| {
            DateTime::parse_from_rfc2822(ival)
                .map(|x| x.with_timezone(&Utc))
                .ok()
        },
        |ival: &str| Utc.datetime_from_str(ival, "%Y %b %d %H:%M:%S%.3f %z").ok(),
        |ival: &str| Utc.datetime_from_str(ival, "%Y-%m-%d %H:%M:%S").ok(),
        |ival: &str| Utc.datetime_from_str(ival, "%Y-%m-%d %H:%M").ok(),
        |ival: &str| {
            DateTime::parse_from_str(ival, "%e/%b/%Y:%T %z")
                .ok()
                .map(|x| x.with_timezone(&Utc))
        },
        |ival: &str| Utc.datetime_from_str(ival, "%e/%b/%Y:%T").ok(),
        |ival: &str| Utc.datetime_from_str(ival, "%a %b %e %T %Y").ok(),
        |ival: &str| {
            NaiveDate::parse_from_str(ival, "%Y-%m-%d")
                .ok()
                .map(|x| x.and_time(NaiveTime::MIN).and_utc())
        },
        |ival: &str| {
            // If the length of the string is > 13 numbers, we should try milliseconds
            match ival.parse::<i64>() {
                Ok(n) => {
                    if ival.len() > 12 {
                        Utc.timestamp_millis_opt(n).single()
                    } else {
                        Utc.timestamp_opt(n, 0).single()
                    }
                }
                Err(_) => None,
            }
        },
    ];
    // Return the first not None value of trying the different parsers on the input_value
    parsers.iter().filter_map(|f| f(&input_value)).next()
}

#[component]
fn DateInputComponent(cx: Scope) -> impl IntoView {
    // create a signal to hold the value
    let (datetime_a, set_datetime_a) = create_signal(cx, None);
    let (datetime_b, set_datetime_b) = create_signal(cx, None);
    let page_load: DateTime<Utc> = Utc::now();
    // set_datetime_b(Some(page_load.clone()));

    view! { cx,
    <fieldset class="flex two">
        <label>
            <input type="text" placeholder="Date time, 2020-01-01"
                on:input=move |ev| {
                    set_datetime_a(parse_input(event_target_value(&ev).trim().to_string()));
                }
            />
        </label>
        <label>
            <input type="text" placeholder="Relative to date time (defaults to page load time)"
                on:input=move |ev| {
                    set_datetime_b(parse_input(event_target_value(&ev).trim().to_string()));
                }
            />
        </label>
    <br/>
    <button style="font-size: x-small;"
    on:click=move |_| {
        set_datetime_a(Some(page_load));
    } >"Use page load time"</button>

    </fieldset>
    <ResultComponent datetime_a=datetime_a datetime_b=datetime_b />
    }
}

#[component]
fn ResultComponent(
    cx: Scope,
    datetime_a: ReadSignal<Option<DateTime<Utc>>>,
    datetime_b: ReadSignal<Option<DateTime<Utc>>>,
) -> impl IntoView {
    let now = Utc::now();
    view! { cx,
    <h2>"Value parsed from input"</h2>
    <table class="pure-table">
        <thead>
            <tr>
                <th>"Formatting"</th>
                <th>"Result"</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>"ISO 8601 / RFC 3339"</td>
                <td>{move || datetime_a.get().map(|nd| nd.to_rfc3339()).unwrap_or(String::from("?"))}</td>
            </tr>
            <tr>
                <td>"RFC 2822"</td>
                <td>{move || datetime_a.get().map(|nd| nd.to_rfc2822()).unwrap_or(String::from("?"))}</td>
            </tr>
            <tr>
                <td>"Local"</td>
                <td>{move || datetime_a.get().map(|nd| format!("{}", nd.format("%c"))).unwrap_or(String::from(""))}</td>
            </tr>
            <tr>
                <td>"UNIX timestamp"</td>
                <td>{move || datetime_a.get().map(|nd| format!("{}", nd.format("%s"))).unwrap_or(String::from(""))}</td>
            </tr>
            <tr>
                <td>"English"</td>
                <td>{move || datetime_a.get().map(|nd| format!("{}", nd.format("%v %r"))).unwrap_or(String::from(""))}</td>
            </tr>
            <tr>
                <td>"Local"</td>
                <td>{move || datetime_a.get().map(|nd| format!("{}", nd.format("%c"))).unwrap_or(String::from(""))}</td>
            </tr>
        </tbody>
    </table>
    <h2>"Calculations with value"</h2>
    <table class="pure-table">
        <thead>
            <tr>
                <th>"Relative to"</th>
                <th>"Date"</th>
                <th>"unit"</th>
                <th>"Time"</th>
                <th>"unit"</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>"0"</td>
                <td class="ra">
                {move || datetime_a.get().map(|nd| nd.format("%Y").to_string()).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{:.3}", nd.year() as f64 * 52.177457)).unwrap_or(String::from(""))} <br/>
                {move || datetime_a.get().map(|nd| format!("{:.3}", nd.year() as f64 * 365.242199)).unwrap_or(String::from(""))} <br/>
                </td>
                <td>
                "years"<br />
                "weeks"<br />
                "days"<br />
                </td>
                <td class="ra">
                {move || datetime_a.get().map(|nd| format!("{:.3}", nd.year() as f64 * 8765.81277)).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{:.3}", nd.year() as f64 * 525948.7662)).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{:.3}", nd.year() as f64 * 31556926.0)).unwrap_or(String::from(""))}<br/>
                </td>
                <td>
                "hours"<br />
                "minutes"<br />
                "seconds"<br />
                </td>
            </tr>
            <tr>
                <td>"1970-01-01 00:00 UTC (epoch)"</td>
                <td />
                <td />
                <td class="ra">
                {move || datetime_a.get().map(|nd| format!("{:.3}", nd.timestamp() as f64 / 3600.0)).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{:.3}", nd.timestamp() as f64 / 60.0)).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{}", nd.timestamp())).unwrap_or(String::from(""))}<br/>
                </td>
                <td>
                "hours"<br />
                "minutes"<br />
                "seconds"<br />
                </td>
            </tr>
            <tr>
                <td class="ra">
                {move || datetime_a.get().map(|nd| nd.format("%F").to_string()).unwrap_or(String::from("?"))}
                </td>
                <td />
                <td />
                <td class="ra">
                {move || datetime_a.get().map(|nd| format!("{:.3}", nd.num_seconds_from_midnight() as f64 / 3600.0)).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{:.3}", nd.num_seconds_from_midnight() as f64 / 60.0)).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{}", nd.num_seconds_from_midnight())).unwrap_or(String::from(""))}<br/>
                </td>
                <td>
                "hours since midnight"<br />
                "minutes since midnight"<br />
                "seconds since midnight"<br />
                </td>
            </tr>

            <tr>
                <td>{move || datetime_b.get().unwrap_or(now).to_rfc3339().to_string()}</td>
                <td class="ra">
                {move || datetime_a.get().map(|nd| format!("{:.3}", nd.signed_duration_since(datetime_b.get().unwrap_or(now)).num_days() as f64/365.242199)).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{}", nd.signed_duration_since(datetime_b.get().unwrap_or(now)).num_weeks())).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{}", nd.signed_duration_since(datetime_b.get().unwrap_or(now)).num_days())).unwrap_or(String::from(""))}
                </td>
                <td>
                "years"<br />
                "weeks"<br />
                "days"<br />
                </td>
                <td class="ra">
                {move || datetime_a.get().map(|nd| format!("{}", nd.signed_duration_since(datetime_b.get().unwrap_or(now)).num_hours())).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{}", nd.signed_duration_since(datetime_b.get().unwrap_or(now)).num_minutes())).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{}", nd.signed_duration_since(datetime_b.get().unwrap_or(now)).num_seconds())).unwrap_or(String::from(""))}<br/>
                </td>
                <td>
                "hours"<br />
                "minutes"<br />
                "seconds"<br />
                </td>
            </tr>
        </tbody>
    </table>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <DateInputComponent/>
    }
}

fn main() {
    let app_element = leptos::document().get_element_by_id("app").unwrap();
    leptos::mount_to(
        app_element.dyn_into::<HtmlElement>().unwrap(),
        |cx| view! { cx, <App/> },
    );
    // leptos::mount_to_body(|cx| view! { cx, <App/> })
}
