use dioxus::prelude::*;

use chrono::{DateTime, Utc};
use futures::future::join_all;
use serde::{Deserialize, Serialize};

fn main() {
    dioxus_web::launch(App);
}
pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || PreviewState::Unset);

    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            width: "100%",
            div {
                width: 50%,
                Stories{}
            }
            div{
                width: 50%,
                Preview {}
            }
        }
    })
}

fn Stories(cx: Scope) -> Element {
    let story = use_future(cx, (), |_| get_stories(10));

    match story.value() {
        Some(Ok(list)) => render! {
            div {
                for story in list {
                    StoryListing { story: story.clone() }
                }
            }
        },
        Some(Err(err)) => render! { "An error occurred while fetching stories {err}"},
        None => render! {"Loading items"},
    }
}

async fn resolve_story(
    full_story: UseRef<Option<StoryPageData>>,
    preview_state: UseSharedState<PreviewState>,
    story_id: i64,
) {
    if let Some(cached) = &*full_story.read() {
        *preview_state.write() = PreviewState::Loaded(cached.clone());
        return;
    }

    *preview_state.write() = PreviewState::Loading;
    if let Ok(story) = get_story(story_id).await {
        *preview_state.write() = PreviewState::Loaded(story.clone());
        *full_story.write() = Some(story);
    }
}

#[inline_props]
fn StoryListing()
