use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}


#[function_component(App)]
fn app() -> Html {
    struct Video {
        id: usize,
        title: String,
        speaker: String,
        url: String
    }
    let videos = vec![
        Video {
            id: 1,
            title: "aaa".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];

    let videos = videos.iter().map(|video| html! {
        <p>{format!("{}: {}", video.speaker, video.title)}</p>
    }).collect::<Html>();
    html! {
        <>
            <h1>{ "RustConf Explorer"}</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
                { videos }
            </div>
            <div>
                <h3>{ "John Doe: Building and breaking things" }</h3>
                <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail"/>
            </div>
        </>
    }
}
