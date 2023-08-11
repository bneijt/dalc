extern crate chrono;

use chrono::prelude::*;
use leptos::*;

///Try various date and datetime formats and return the first one that works
fn parse_input(input_value: String) -> Option<DateTime<Utc>> {
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
        |ival: &str| DateTime::parse_from_str(ival, "%e/%b/%Y:%T %z").ok().map(|x| x.with_timezone(&Utc)),
        |ival: &str| Utc.datetime_from_str(ival, "%e/%b/%Y:%T").ok(),
        |ival: &str| Utc.datetime_from_str(ival, "%a %b %e %T %Y").ok(),
        |ival: &str| NaiveDate::parse_from_str(ival, "%Y-%m-%d").ok().map(|x| x.and_time(NaiveTime::MIN).and_utc()),
        |ival: &str| Utc.datetime_from_str(ival, "%s").ok(),
    ];
    // Return the first not None value of trying the different parsers on the input_value
    parsers.iter().filter_map(|f| f(&input_value)).next()
}

#[component]
fn DateInputComponent(cx: Scope) -> impl IntoView {
    // create a signal to hold the value
    let (datetime_a, set_datetime_a) = create_signal(cx, None);
    let page_load: DateTime<Utc> = Utc::now();

    view! { cx,
    <fieldset class="flex two">
        <label>
        <input type="text" placeholder="Date time A, 2020-01-01"
            on:input=move |ev| {
                // set_naive_date(parse_date(event_target_value(&ev)));
                set_datetime_a(parse_input(event_target_value(&ev)));
            }
        />
        </label>

    </fieldset>
     <button
        style="font-size: x-small;"
        on:click=move |_| {
            set_datetime_a(Some(page_load));
        }
    >"Use page load time"</button>
    <br/>
    <ResultComponent datetime_a=datetime_a/>
    }
}

#[component]
fn ResultComponent(cx: Scope, datetime_a: ReadSignal<Option<DateTime<Utc>>>) -> impl IntoView {
    let now: DateTime<Utc> = Utc::now();
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
                <th>"Time"</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>"0"</td>
                <td>
                {move || datetime_a.get().map(|nd| nd.format("%Y years").to_string()).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{} weeks", nd.year() as f64 * 52.177457)).unwrap_or(String::from(""))} <br/>
                {move || datetime_a.get().map(|nd| format!("{} days", nd.year() as f64 * 365.242199)).unwrap_or(String::from(""))} <br/>
                </td>
                <td>
                {move || datetime_a.get().map(|nd| format!("{} hours", nd.year() as f64 * 8765.81277)).unwrap_or(String::from(""))} <br/>
                {move || datetime_a.get().map(|nd| format!("{} minutes", nd.year() as f64 * 525948.7662)).unwrap_or(String::from(""))} <br/>
                {move || datetime_a.get().map(|nd| format!("{} seconds", nd.year() as f64 * 31556926.0)).unwrap_or(String::from(""))} <br/>
                </td>
            </tr>
            <tr>
                <td>"1970-01-01 00:00 UTC (epoch)"</td>
                <td />
                <td>{move || datetime_a.get().map(|nd| nd.format("%s seconds").to_string()).unwrap_or(String::from(""))}</td>
            </tr>
            <tr>
                <td>
                {move || datetime_a.get().map(|nd| nd.format("%F").to_string()).unwrap_or(String::from("?"))}
                </td>
                <td />
                <td>
                {move || datetime_a.get().map(|nd| format!("{} seconds since midnight", nd.num_seconds_from_midnight())).unwrap_or(String::from(""))}<br/>
                </td>
                
            </tr>

            <tr>
                <td>{now.to_rfc3339().to_string()}</td>
                <td>
                {move || datetime_a.get().map(|nd| format!("{} years", nd.signed_duration_since(now).num_days() as f64/365.242199)).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{} weeks", nd.signed_duration_since(now).num_weeks())).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{} days", nd.signed_duration_since(now).num_days())).unwrap_or(String::from(""))}
                </td>
                <td>
                {move || datetime_a.get().map(|nd| format!("{} hours", nd.signed_duration_since(now).num_hours())).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{} minutes", nd.signed_duration_since(now).num_minutes())).unwrap_or(String::from(""))}<br/>
                {move || datetime_a.get().map(|nd| format!("{} seconds", nd.signed_duration_since(now).num_seconds())).unwrap_or(String::from(""))}<br/>
                </td>
            </tr>
        </tbody>
    </table>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <main class="flex two center">
        <div>
                <h1>"Dalc"</h1>
                <p>
                "Simple date calculator, just fill in a datetime string."
                </p><p>
                " Current input formats include ISO 8601, RFC 3339, RFC 2822, epoch timestamp, and others."
                " Put any supported date/time format string into the box below, and it will immediately calculate some values and different formatting:"
                </p>
                <DateInputComponent/>
                <p><a href="https://twitter.com/intent/tweet?url=https%3A//bneijt.nl/pr/dalc&amp;text=%40bneijt%20%23feedback">"Twitter for feedback"</a></p>
        </div>
        </main>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}