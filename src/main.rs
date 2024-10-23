use leptos::*;
use leptos_use::{FacingMode, MediaTrackConstraintsBuilder, use_user_media_with_options, UseUserMediaOptions, UseUserMediaReturn};

#[component]
fn Demo() -> impl IntoView {
    let video_ref = create_node_ref::<leptos::html::Video>();

    let UseUserMediaReturn {
        stream,
        enabled,
        set_enabled,
        ..
    } = use_user_media_with_options(
        UseUserMediaOptions::default()
          .video(
              MediaTrackConstraintsBuilder::new()
                .value(false.into())
                .build()
          )
          .audio(
              MediaTrackConstraintsBuilder::new()
                .value(true.into())
                .build()
          ),
    );

    create_effect(move |_| {
        match stream.get() {
            Some(Ok(s)) => {
                video_ref.get().map(|v| v.set_src_object(Some(&s)));
                return;
            }
            Some(Err(e)) => logging::error!("Failed to get media stream: {:?}", e),
            None => logging::log!("No stream yet"),
        }

        video_ref.get().map(|v| v.set_src_object(None));
    });

    view! {
        <div class="flex flex-col gap-4 text-center">
            <div>
                <button on:click=move |_| set_enabled(
                    !enabled(),
                )>{move || if enabled() { "Stop" } else { "Start" }} video</button>
            </div>

            <div>
                <video
                    node_ref=video_ref
                    controls=false
                    autoplay=true
                    muted=true
                    class="h-96 w-auto"
                ></video>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| {
        view! { <Demo/> }
    })
}