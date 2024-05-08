use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::building::{Floor, Room};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/hvacrux.css"/>

        // sets the document title
        <Title text="HVACrux"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let floors = vec![
        Floor {
            rooms: vec![Room {
                length: 5.0,
                width: 5.0,
                height: 3.0,
                window_area: 2.0,
                num_people: 2,
                lighting_load: 100.0,
                appliance_load: 200.0,
            }],
        },
        Floor {
            rooms: vec![
                Room {
                    length: 6.0,
                    width: 5.0,
                    height: 3.0,
                    window_area: 2.0,
                    num_people: 2,
                    lighting_load: 100.0,
                    appliance_load: 200.0,
                },
                Room {
                    length: 5.0,
                    width: 5.0,
                    height: 3.0,
                    window_area: 2.0,
                    num_people: 2,
                    lighting_load: 123.4,
                    appliance_load: 200.2,
                },
            ],
        },
    ];
    view! {
        <Floors floors=&floors />
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[component]
fn Floors<'a>(floors: &'a Vec<Floor>) -> impl IntoView {
    // let form_data = use_context().expect("No FormData found");

    view! {
        <fieldset>
            <legend>"Floors"</legend>
            <div>
                <input type="number" value={floors.len()} min="1" placeholder="Number of floors" />
            </div>
            <div>
                {floors.iter().enumerate().map(|(index, floor)| { view! { <RoomList floor=&floor index=index /> }}).collect_view()}
            </div>
        </fieldset>
    }
}

#[component]
pub fn RoomList<'a>(floor: &'a Floor, index: usize) -> impl IntoView {
    // let form_data = use_context().expect("No FormData found");

    view! {
        <fieldset>
            <legend>{"Floor #"} {index + 1}</legend>
            <div>
                <input type="number" min="1" value={floor.rooms.len()} placeholder="Number of rooms" />
            </div>
            <div>
                {floor.rooms.iter().enumerate().map(|(index, room)| { view! { <RoomDetails room=&room index=index /> }}).collect_view()}
            </div>
        </fieldset>
    }
}

#[component]
pub fn RoomDetails<'a>(room: &'a Room, index: usize) -> impl IntoView {
    view! {
        <fieldset>
            <legend>{"Room #"} {index + 1}</legend>
            <div>
                <label>"Length (m)"</label>
                <input type="number" step="0.01" value={room.length} placeholder="Length" />
            </div>
            <div>
                <label>"Width (m)"</label>
                <input type="number" step="0.01" value={room.width} placeholder="Width" />
            </div>
            <div>
                <label>"Height (m)"</label>
                <input type="number" step="0.01" value={room.height} placeholder="Height" />
            </div>
            <div>
                <label>"Window Area (m²)"</label>
                <input type="number" step="0.01" value={room.window_area} placeholder="Window Area" />
            </div>
            <div>
                <label>"Number of People"</label>
                <input type="number" min="0" value={room.num_people} placeholder="Number of People" />
            </div>
            <div>
                <label>"Lighting Load (W)"</label>
                <input type="number" min="0" value={room.lighting_load} placeholder="Lighting Load" />
            </div>
            <div>
                <label>"Appliance Load (W)"</label>
                <input type="number" min="0" value={room.appliance_load} placeholder="Appliance Load" />
            </div>
        </fieldset>
    }
}
