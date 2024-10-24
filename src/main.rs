use leptos::*;
use leptos_use::{AudioTrackConstraints, ConstraintBool, ConstraintDouble, ConstraintFacingMode, ConstraintULong, FacingMode, use_user_media_with_options, UseUserMediaOptions, UseUserMediaReturn, VideoTrackConstraints};

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
              VideoTrackConstraints::new()
                .frame_rate(ConstraintDouble::default().exact(30f64))
                .facing_mode(FacingMode::User)
                .facing_mode(ConstraintFacingMode::default().exact(FacingMode::Environment))
          )
          .audio(
              AudioTrackConstraints::new()
                .device_id("USB Audio Device")
                // TODO: implement
                // .device_ids(vec!["default".into(), "foo".into()])
                .auto_gain_control(ConstraintBool::default().exact(true))
                .channel_count(ConstraintULong::default().min(1).max(3))
                .echo_cancellation(true)
                .noise_suppression(true)

          )
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